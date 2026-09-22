use std::{fs, io, os::unix::{fs::PermissionsExt, net::UnixListener}};

use ecodynamic_api::LETTERBOX_PATH;

pub fn open() -> io::Result<UnixListener> {
    if fs::metadata(LETTERBOX_PATH).is_ok() {
        log::warn!("Closing previous socket: {}", LETTERBOX_PATH);
        fs::remove_file(LETTERBOX_PATH)?;
    }
    log::info!("Creating socket: {}", LETTERBOX_PATH);
    let listener = UnixListener::bind(LETTERBOX_PATH)?;
    fs::set_permissions(LETTERBOX_PATH, fs::Permissions::from_mode(0o666))?;
    Ok(listener)
}

pub fn close() -> io::Result<()> {
    log::info!("Closing socket: {}", LETTERBOX_PATH);
    fs::remove_file(LETTERBOX_PATH)
}
