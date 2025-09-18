use log::LevelFilter;
use simple_logger::SimpleLogger;

pub fn set_verbose(verbose: bool) {
    let log_level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let _ = SimpleLogger::new().with_level(log_level).init();
}