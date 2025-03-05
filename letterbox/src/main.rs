mod message;

use std::collections::HashMap;
use std::os::unix::net::UnixListener;
use std::io::{self, Read, Write};
use std::fs;

//use controller::corridor_controller::Controller;
use controller::delta_controller::Controller;
use message::Message;

const SOCKET_PATH: &str = "/tmp/mtdynamic_letterbox";

#[derive(Default)]
pub struct Letterbox<const N: usize> {
    letterboxes: HashMap<(i32, i32), (Controller, Samples<N>)>,
}

impl<const N: usize> Letterbox<N> {
    pub fn update(&mut self, msg: Message) -> i32 {
        if let Some((controller, samples)) = self.letterboxes.get_mut(&(msg.pid, msg.fid)) {
            samples.push(msg.val);

            if samples.len >= N {
                controller.adjust_threads(samples.take())
            } else {
                controller.threads()
            }
        } else {
            let controller = Controller::new(16);
            let samples = Samples::from(msg.val);
            self.letterboxes.insert((msg.pid, msg.fid), (controller, samples));
            16
        }
    }
}

struct Samples<const N: usize> {
    elems: [f32; N],
    len: usize,
}

impl<const N: usize> Samples<N> {
    fn take(&mut self) -> Vec<f32> {
        self.elems.to_vec()
    }

    fn push(&mut self, value: f32) {
        assert!(self.len < N);
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

fn main() -> io::Result<()> {
    // Remove any existing socket file
    if fs::metadata(SOCKET_PATH).is_ok() {
        fs::remove_file(SOCKET_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(SOCKET_PATH)?;
    println!("Server listening on {}", SOCKET_PATH);

    // Create a letterbox
    let mut letterbox: Letterbox<10> = Letterbox::default();
    let mut buffer = [0u8; 12];

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // Read from stream
                stream.read_exact(&mut buffer)?;
                let msg = Message::from(buffer);
                println!("Recv: {:?}", msg);

                // Update letterbox
                let threads = letterbox.update(msg);

                // Write to stream
                stream.write_all(&threads.to_ne_bytes())?;
                println!("Send: {}", threads);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    unreachable!()
}
