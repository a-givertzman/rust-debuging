use std::{env, sync::Once};

#[allow(dead_code)]
static INIT: Once = Once::new();

///
/// 
#[allow(dead_code)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
///
/// 
#[allow(dead_code)]
pub enum Backtrace {
    Full,
    Short,
}

///
/// Call DebugSession::init() to initialize logging
#[allow(dead_code)]
pub struct DebugSession {}
///
/// 
impl DebugSession {
    ///
    /// Initialize debug session on first call, all next will be ignored
    #[allow(dead_code)]
    pub fn init(log_level: LogLevel, backtrace: Backtrace) {
        INIT.call_once(|| {
            let log_style = "always";
            let log_level = match log_level {
                LogLevel::Off => "off",
                LogLevel::Error => "error",
                LogLevel::Warn => "warn",
                LogLevel::Info => "info",
                LogLevel::Debug => "debug",
                LogLevel::Trace => "trace",
                // _ => "debug",
            };
            let backtrace = match backtrace {
                Backtrace::Full => "full",
                Backtrace::Short => "short",
            };
            env::set_var("RUST_LOG", log_level);  // off / error / warn / info / debug / trace
            assert_eq!(env::var("RUST_LOG"), Ok(log_level.to_string()), "Set env RUST_LOG={} failed", log_level);
            env::set_var("RUST_BACKTRACE", backtrace);
            assert_eq!(env::var("RUST_BACKTRACE"), Ok(backtrace.to_string()), "Set env RUST_BACKTRACE={} failed", backtrace);
            env::set_var("RUST_LOG_STYLE", log_style);     // auto / always / never
            assert_eq!(env::var("RUST_LOG_STYLE"), Ok(log_style.to_string()), "Set env RUST_LOG_STYLE={} failed", log_style);
            match env_logger::builder().is_test(true).try_init() {
            // match builder.is_test(true).try_init() {
                Ok(_) => {
                    println!("DebugSession.init | Ok");
                    println!("DebugSession.init | RUST_LOG = {:?}", env::var("RUST_LOG"));
                    println!("DebugSession.init | RUST_BACKTRACE = {:?}", env::var("RUST_BACKTRACE"));
                    println!("DebugSession.init | RUST_LOG_STYLE = {:?}", env::var("RUST_LOG_STYLE"));
                    println!("");
                },
                Err(err) => {
                    println!("DebugSession.init | error: {:?}", err)
                },
            };
        })
    }
}
