use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::thread::sleep;
use std::time::Duration;

use controller::*;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect(MTD_LETTERBOX_PATH)?;

    const REGION_UID: i32 = 0;

    loop {
        // Write request to stream
        let req = Request { region_uid: REGION_UID, problem_size: 0 };
        println!("Req: {:?}", req);
        stream.write_all(&req.to_bytes())?;

        // Read from stream
        let mut buffer = [0u8; LocalDemand::SIZE];
        stream.read_exact(&mut buffer)?;
        let demand = LocalDemand::from(buffer);
        println!("Recv: {:?}", demand);

        sleep(Duration::from_secs(1));

        // Write sample to stream
        let sample = Sample {
            region_uid: REGION_UID,
            runtime: 42.37,
            usertime: 16.0 * 45.11,
            energy: 98.30
        };
        println!("Send: {:?}", sample);
        stream.write_all(&sample.to_bytes())?;
    }
}
