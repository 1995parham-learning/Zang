use std::str::FromStr;

use log::LevelFilter;

mod config;

fn main() {
    // return the error with ? when using tokio and anyhow
    let cfg = config::Settings::init();
    setup_logger(&cfg.as_ref().unwrap().log);

    if cfg.is_err() {
        log::error!("failed to initialize config: {}", cfg.err().unwrap());
    } else {
        log::debug!("initialized config: {:?}", &cfg);
    }
}

fn setup_logger(s: &config::Log) {
    env_logger::builder()
        .filter(
            None,
            LevelFilter::from_str(&s.level).unwrap_or(LevelFilter::Error),
        )
        .init();
}
