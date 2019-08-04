fn main() {
    femme::ndjson::Logger::new()
        .start(log::LevelFilter::Trace)
        .unwrap();
    log::error!("Buffer has to be 16 bytes in length");
    log::warn!("Unauthorized access attempt on /login");
    log::info!("Listening on port 8080");
    log::debug!("Getting String as bson value type");
    log::trace!("Called public function get_type");
}
