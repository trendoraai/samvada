use log::debug;
use simplelog::{
    CombinedLogger, ConfigBuilder, LevelFilter, LevelPadding, ThreadLogMode, WriteLogger,
};
use std::fs::File;
use std::path::{Path, PathBuf};
use time::macros::format_description;
use time::UtcOffset;
use chrono::Local;

/// Sets up logging configuration based on the provided file path or defaults to "samvada.log".
pub fn setup_logging(file_path: Option<&str>) -> PathBuf {
    let log_path = if let Some(file_path) = file_path {
        let path = Path::new(file_path);
        let stem = path.file_stem().unwrap_or_default();
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        parent.join(format!("{}.log", stem.to_str().unwrap()))
    } else {
        // Default log file name if no file_path is provided
        PathBuf::from("samvada.log")
    };

    let offset_in_sec = Local::now().offset().local_minus_utc();

    let local_offset = UtcOffset::from_whole_seconds(offset_in_sec).unwrap_or_else(|_| {
        eprintln!("Invalid offset: {}. Defaulting to UTC", offset_in_sec);
        UtcOffset::UTC
    });

    debug!("Using UTC offset: {:?}", local_offset);

    let config = ConfigBuilder::new()
        .set_thread_mode(ThreadLogMode::Both)
        .set_level_padding(LevelPadding::Right)
        .set_time_format_custom(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3] [offset_hour sign:mandatory]:[offset_minute]"
        ))
        .set_time_offset(local_offset)
        .build();

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        config,
        File::create(&log_path).expect("Failed to create log file"),
    )])
    .expect("Failed to initialize loggers");

    log_path
}