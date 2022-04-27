pub fn hello() {
    if log::log_enabled!(log::Level::Trace) {
        log::trace!("test");
    } else if log::log_enabled!(log::Level::Debug) {
        log::debug!("test");
    } else if log::log_enabled!(log::Level::Info) {
        log::info!("test");
    } else if log::log_enabled!(log::Level::Warn) {
        log::warn!("test");
    } else if log::log_enabled!(log::Level::Error) {
        log::error!("test");
    }
}
