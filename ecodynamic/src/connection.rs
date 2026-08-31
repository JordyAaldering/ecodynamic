use std::{
    io::{self, BufRead, BufReader, Write},
    os::unix::net::UnixStream,
};

use controller::{AppCapabilities, LETTERBOX_PATH};

pub struct Connection {
    stream: UnixStream,
    reader: BufReader<UnixStream>,
}

impl Connection {
    pub fn connect(max_threads: u16) -> io::Result<Self> {
        let pid = std::process::id() as i32;
        let capabilities = AppCapabilities::new(pid, max_threads);

        let mut stream = UnixStream::connect(LETTERBOX_PATH)?;
        let reader = BufReader::new(stream.try_clone()?);
        serde_json::to_writer(&mut stream, &capabilities)?;
        stream.write_all(b"\n")?;

        Ok(Self { stream, reader })
    }

    pub fn read<T>(&mut self) -> io::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut line = String::new();
        let read = self.reader.read_line(&mut line)?;

        if read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Received empty response from controller, is it still running?",
            ));
        }

        serde_json::from_str(line.trim_end()).map_err(io::Error::other)
    }

    pub fn write<T>(&mut self, value: &T) -> io::Result<()>
    where
        T: serde::ser::Serialize,
    {
        serde_json::to_writer(&mut self.stream, value)?;
        self.stream.write_all(b"\n")
    }
}
