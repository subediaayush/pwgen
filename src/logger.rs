use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn set_verbose(verbose: bool) {
    let mut builder = Builder::new();
    builder.format(|buf, record| {
        writeln!(buf, "{}", record.args())
    });
    builder.filter_level(if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });
    let _ = builder.try_init();
}