use std::mem;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::thread::sleep;
use std::time::Duration;

use controller::{Demand, Sample};
use letterbox::MTD_LETTERBOX_PATH;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect(MTD_LETTERBOX_PATH)?;

    loop {
        // Create a sample
        let incoming = Sample {
            max_threads: 16,
            num_threads: 16,
            region_uid: 0,
            runtime: 42.37,
            usertime: 16.0 * 45.11,
            energy: 98.30
        };

        // Write to stream
        println!("Send: {:?}", incoming);
        stream.write_all(&incoming.to_bytes())?;

        // Read from stream
        let mut buffer = [0u8; mem::size_of::<Demand>()];
        stream.read_exact(&mut buffer)?;
        let outgoing = Demand::from(buffer);
        println!("Recv: {:?}", outgoing);

        sleep(Duration::from_secs(1));
    }
}
