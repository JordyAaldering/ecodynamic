use std::{fs, os::unix::{fs::PermissionsExt, net::UnixListener}};

use ecodynamic_api::LETTERBOX_PATH;

pub fn open() -> UnixListener {
    if fs::metadata(LETTERBOX_PATH).is_ok() {
        log::warn!("Closing previous socket: {}", LETTERBOX_PATH);
        fs::remove_file(LETTERBOX_PATH).expect("Could not close socket");
    }
    log::info!("Creating socket: {}", LETTERBOX_PATH);
    let listener = UnixListener::bind(LETTERBOX_PATH)
        .expect("Could not create socket");
    fs::set_permissions(LETTERBOX_PATH, fs::Permissions::from_mode(0o666))
        .expect("Failed to set socket permissions");
    listener
}

pub fn close() {
    log::info!("Closing socket: {}", LETTERBOX_PATH);
    fs::remove_file(LETTERBOX_PATH).expect("Could not close socket");
}
