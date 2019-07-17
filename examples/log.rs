use log::info;

fn setup_logger() {
    femme::Logger::new().start(log::LevelFilter::Trace).unwrap();
}

fn main() {
    setup_logger();
    info!("Hello planet");
    info!("nori cat!");
}
