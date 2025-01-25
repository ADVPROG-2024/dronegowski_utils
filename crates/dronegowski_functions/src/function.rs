use std::fs::File;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};

pub fn generate_unique_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_millis() as u64
}

pub fn simple_log() {
    let log_level = LevelFilter::Info;
    let _logger = WriteLogger::init(
        log_level,
        ConfigBuilder::new().set_thread_level(log_level).build(),
        File::create("output.log").expect("Could not create log file"),
    );
}