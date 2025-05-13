use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

use mtd_server::*;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

fn handle_client(mut stream: UnixStream, client_id: usize) -> io::Result<()> {
    let mut lbs = Letterbox::new(|req| ControllerType::build(req));

    let mut buffer = [0u8; Sample::SIZE];

    let mut log = if let Some(path) = &CONFIG.lock().log_path {
        let path = path.join(format!("client{:02}.csv", client_id));
        println!("Creating log file at {:?}", path);
        let file = File::create_new(path)?;
        let mut w = BufWriter::new(file);
        w.write("uid,threads,runtime,usertime,energy\n".as_bytes())?;
        Some(w)
    } else {
        None
    };

    loop {
        // Try to read from the stream
        match stream.read(&mut buffer) {
            Ok(Request::SIZE) => {
                let buf: [u8; Request::SIZE] = buffer[0..Request::SIZE].try_into().unwrap();
                let req = Request::from(buf);
                debug_println!("Read: {:?}", req);

                // Update letterbox

                let (_, controller) = lbs.letterbox.entry(req.region_uid)
                    .or_insert_with(|| (SampleVec::new(), (lbs.build_fn)(req)));
                let num_threads = controller.num_threads();
                let demand = Demand { num_threads };

                // Write to stream
                debug_println!("Send: {:?}", demand);
                let buf: [u8; Demand::SIZE] = demand.to_bytes();
                stream.write_all(&buf)?;
            }
            Ok(Sample::SIZE) => {
                let sample = Sample::from(buffer);

                debug_println!("Recv: {:?}", sample);

                let (samples, controller) = lbs.letterbox.get_mut(&sample.region_uid).unwrap();

                if let Some(w) = &mut log {
                    w.write_fmt(format_args!("{},{},{},{},{}\n",
                        sample.region_uid,
                        controller.num_threads(),
                        sample.runtime,
                        sample.usertime,
                        sample.energy)
                    )?;
                }

                if let Some(samples) = samples.push_until_full(sample) {
                    controller.evolve(samples);
                }
            }
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                eprintln!("Invalid message size: {}", n);
                break;
            }
            Err(e) => {
                eprintln!("Client disconnected: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Check if log directory exists
    if let Some(path) = &CONFIG.lock().log_path {
        let path = fs::canonicalize(path)?;
        println!("Writing logs to {:?}", path);
    }

    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    println!("Server listening on {}", MTD_LETTERBOX_PATH);

    if CONFIG.lock().single {
        let stream = listener.incoming().next().unwrap();
        match stream {
            Ok(stream) => handle_client(stream, 0)?,
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    } else {
        let mut client_count: usize = 0;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    client_count += 1;
                    std::thread::spawn(move || {
                        handle_client(stream, client_count)
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }

    // Remove socket file
    fs::remove_file(MTD_LETTERBOX_PATH)?;

    Ok(())
}
