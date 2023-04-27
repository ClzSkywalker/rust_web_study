use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

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
        .init();
    ()
}
