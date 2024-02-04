use chrono::Local;
use colored::{ColoredString, Colorize};
use fern::Dispatch;
use log::{Level, LevelFilter};

fn lvl_color(level: Level) -> ColoredString {
    match level {
        Level::Warn => "WARN".yellow().bold(),
        Level::Info => "INFO".bright_blue().bold(),
        Level::Error => "ERROR".red().bold(),
        Level::Debug => "DEBUG".bright_green().bold(),
        Level::Trace => "TRACE".purple().bold(),
    }
}

fn iologging(verbose: LevelFilter) -> Result<Dispatch, Box<dyn std::error::Error>> {
    Ok(Dispatch::new()
        .format(move |out, msg, record| {
            let now = Local::now();
            out.finish(format_args!(
                "< [{:5}] {:8} {:3}{:4}> {}",
                lvl_color(record.level()),
                now.format("%H:%M:%S"),
                record.target(),
                if verbose == LevelFilter::Debug || verbose == LevelFilter::Trace {
                    format!(" {}", record.line().unwrap())
                } else {
                    "".to_string()
                },
                msg,
            ))
        })
        .level(verbose)
        .chain(std::io::stdout()))
}
fn flogging() -> Result<Dispatch, Box<dyn std::error::Error>> {
    Ok(Dispatch::new()
        .format(|out, msg, record| {
            let now = Local::now();
            out.finish(format_args!(
                "< [{:5}] {:8} {:3}{:4}> {}",
                record.level().to_string(),
                now.format("%H:%M:%S"),
                record.target(),
                record.line().unwrap(),
                msg,
            ))
        })
        .level(LevelFilter::Debug)
        .chain(fern::log_file("logs/main.log")?))
}
pub fn setup_logging(verbose: LevelFilter) {
    let file_logging = flogging().unwrap();
    let cmd_logging = iologging(verbose).unwrap();
    Dispatch::new()
        .chain(file_logging)
        .chain(cmd_logging)
        .apply()
        .unwrap();
}
