use std::{
    fmt,
    fs,
    io::{self, BufRead, BufReader, Write},
    os::unix::{fs::PermissionsExt, net::{UnixListener, UnixStream}},
};

use ecodynamic_api::*;

pub enum Response {
    Request(Request),
    Sample(Sample),
    Disconnect,
}

pub fn open() -> io::Result<UnixListener> {
    if fs::metadata(LETTERBOX_PATH).is_ok() {
        log::warn!("Closing previous socket: {LETTERBOX_PATH}");
        fs::remove_file(LETTERBOX_PATH)?;
    }
    log::info!("Creating socket: {LETTERBOX_PATH}");
    let listener = UnixListener::bind(LETTERBOX_PATH)?;
    fs::set_permissions(LETTERBOX_PATH, fs::Permissions::from_mode(0o666))?;
    Ok(listener)
}

pub fn close() -> io::Result<()> {
    log::info!("Closing socket: {LETTERBOX_PATH}");
    fs::remove_file(LETTERBOX_PATH)
}

pub fn response(rdr: &mut BufReader<UnixStream>) -> io::Result<Response> {
    let mut line = String::new();
    let bytes_read = rdr.read_line(&mut line)?;
    if bytes_read == 0 {
        log::info!("Client disconnected");
        Ok(Response::Disconnect)
    } else {
        // Note that we must check for `Sample` first, as a `Request` may be seen as a `Sample` if it only contains `region`.
        if let Ok(sample) = serde_json::from_str::<Sample>(&line) {
            log::trace!("POST: {sample:?}");
            Ok(Response::Sample(sample))
        } else if let Ok(request) = serde_json::from_str::<Request>(&line) {
            log::trace!("GET: {request:?}");
            Ok(Response::Request(request))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid JSON response: {line}"))
            )
        }
    }
}

pub fn write<T: serde::Serialize>(stream: &mut UnixStream, message: &T) -> io::Result<()>
where
    T: fmt::Debug + serde::Serialize,
{
    log::trace!("PUT: {message:?}");
    serde_json::to_writer(&mut *stream, message).map_err(io::Error::other)?;
    stream.write_all(b"\n")
}
