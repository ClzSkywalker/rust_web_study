use common::log::log::init_log;
use tracing::{info, error, trace, debug, warn};


#[test]
fn log_test(){
    init_log("./logs");
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}