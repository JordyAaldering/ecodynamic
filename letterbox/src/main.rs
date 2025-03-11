use std::collections::HashMap;
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{self, Read, Write};
use std::{fs, mem};

//use controller::corridor_controller::Controller;
use controller::delta_controller::Controller;
use letterbox::{Incoming, Outgoing};

const MTD_LETTERBOX_PATH: &str = "/tmp/mtd_letterbox";

#[derive(Default)]
pub struct Letterbox<const N: usize> {
    letterboxes: HashMap<i32, (Controller, Samples<N>)>,
}

impl<const N: usize> Letterbox<N> {
    pub fn update(&mut self, msg: Incoming) -> Outgoing {
        let threads = if let Some((controller, samples)) = self.letterboxes.get_mut(&msg.uid) {
            samples.push(msg.val);

            if samples.len >= N {
                controller.adjust_threads(samples.take())
            } else {
                controller.threads()
            }
        } else {
            let max_threads = num_cpus::get() as i32;
            let controller = Controller::new(max_threads);
            let samples = Samples::from(msg.val);
            self.letterboxes.insert(msg.uid, (controller, samples));
            max_threads
        };
        Outgoing { threads }
    }
}

struct Samples<const N: usize> {
    elems: [f32; N],
    len: usize,
}

impl<const N: usize> Samples<N> {
    fn take(&mut self) -> Vec<f32> {
        self.len = 0;
        self.elems.to_vec()
    }

    fn push(&mut self, value: f32) {
        self.elems[self.len] = value;
        self.len += 1;
    }
}

impl<const N: usize> From<f32> for Samples<N> {
    fn from(value: f32) -> Self {
        let mut elems = [0f32; N];
        elems[0] = value;
        Self { elems, len: 1 }
    }
}

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    let mut letterbox: Letterbox<10> = Letterbox::default();
    let mut buffer = [0u8; mem::size_of::<Incoming>()];

    loop {
        // Try to read from the stream
        match stream.read_exact(&mut buffer) {
            Ok(()) => {
                let incoming = Incoming::from(buffer);
                println!("Recv: {:?}", incoming);

                // Update letterbox
                let outgoing = letterbox.update(incoming);

                // Write to stream
                println!("Send: {:?}", outgoing);
                let buf: [u8; 4] = outgoing.to_bytes();
                stream.write_all(&buf)?;
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
    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    println!("Server listening on {}", MTD_LETTERBOX_PATH);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    unreachable!()
}
