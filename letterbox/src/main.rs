use std::collections::HashMap;
use std::os::unix::net::UnixListener;
use std::io::{Read, Write};
use std::fs;

//use controller::corridor_controller::Controller;
use controller::delta_controller::Controller;

const SOCKET_PATH: &str = "/tmp/rust_unix_socket";

#[derive(Default)]
pub struct Letterbox<const N: usize> {
    /// Mapping from (process id, function id) to energy/runtime values.
    letterboxes: HashMap<(i32, i32), (Controller, Samples<N>)>,
}

impl<const N: usize> Letterbox<N> {
    pub fn update(&mut self, pid: i32, fid: i32, value: i32) -> i32 {
        if let Some((controller, samples)) = self.letterboxes.get_mut(&(pid, fid)) {
            samples.push(value);

            if samples.len >= N {
                controller.adjust_threads(samples.take().map(|x| x as f32).to_vec())
            } else {
                controller.threads()
            }
        } else {
            let controller = Controller::new(16);
            let samples = Samples::from(value);
            self.letterboxes.insert((pid, fid), (controller, samples));
            16
        }
    }
}

struct Samples<const N: usize> {
    elems: [i32; N],
    len: usize,
}

impl<const N: usize> Samples<N> {
    fn take(&mut self) -> [i32; N] {
        let res = self.elems;
        self.elems = [0; N];
        res
    }

    fn push(&mut self, value: i32) {
        assert!(self.len < N);
        self.elems[self.len] = value;
        self.len += 1;
    }
}

impl<const N: usize> From<i32> for Samples<N> {
    fn from(value: i32) -> Self {
        let mut elems = [0; N];
        elems[0] = value;
        Self { elems, len: 1 }
    }
}

fn main() -> std::io::Result<()> {

    // Remove any existing socket file
    if fs::metadata(SOCKET_PATH).is_ok() {
        fs::remove_file(SOCKET_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(SOCKET_PATH)?;
    println!("Server listening on {}", SOCKET_PATH);

    let mut letterbox: Letterbox<10> = Letterbox::default();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client connected");

                let mut buffer = [0u8; 3 * std::mem::size_of::<i32>()];

                // Read from stream
                stream.read_exact(&mut buffer)?;
                let pid = i32::from_ne_bytes(buffer[0..4].try_into().unwrap());
                let fid = i32::from_ne_bytes(buffer[0..4].try_into().unwrap());
                let value = i32::from_ne_bytes(buffer[0..4].try_into().unwrap());
                println!("Received: ({}, {}) -> {}", pid, fid, value);

                // Update letterbox
                let threads = letterbox.update(pid, fid, value);

                // Write to stream
                stream.write_all(&threads.to_ne_bytes())?;
                println!("Set thread-count of ({}, {}) to {}", pid, fid, threads);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    fs::remove_file(SOCKET_PATH)
}
