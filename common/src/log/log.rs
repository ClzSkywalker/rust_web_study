use chrono::Local;
use tracing::{info, log::error, Level};
use tracing_appender::rolling;
use tracing_subscriber::fmt::{time::FormatTime, writer::MakeWriterExt};

#[derive(Debug)]
struct MyTimeFormat;

impl FormatTime for MyTimeFormat {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        write!(w, "{}", now.to_string())
    }
}

/**
 * @Author         : ClzSkywalker
 * @Date           : 2023-04-28
 * @Description    : 初始化 Log
 * @param           {*} path
 * @return          {*}
 */
pub fn init_log(path: &str) {
    let info_file = rolling::daily(path, "info")
        .with_max_level(Level::DEBUG)
        .with_min_level(Level::INFO);
    let err_file = rolling::daily(path, "error").with_max_level(Level::WARN);
    let all_files = info_file.and(err_file);

    tracing_subscriber::fmt()
        .with_writer(all_files)
        .with_ansi(false)
        .with_max_level(Level::TRACE)
        .with_timer(MyTimeFormat)
        .init();
    ()
}

pub fn info(t: String) {
    info!(t);
}

pub fn error(t: String) {
    error!("{}", t);
}
