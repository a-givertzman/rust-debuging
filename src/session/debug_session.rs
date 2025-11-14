use std::{env, sync::Once};
use tracing_subscriber::{filter::{LevelFilter, Targets}, layer::SubscriberExt, util::SubscriberInitExt};

#[allow(dead_code)]
static INIT: Once = Once::new();

///
/// Levels are typically used to implement filtering that determines which spans and events are enabled.
/// Depending on the use case, more or less verbose diagnostics may be desired.
/// - When running in development, DEBUG-level traces may be enabled by default.
/// - When running in production, only INFO-level and lower traces might be enabled.
#[allow(dead_code)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
impl Into<LevelFilter> for LogLevel {
    fn into(self) -> LevelFilter {
        match self {
            LogLevel::Off => LevelFilter::OFF,
            LogLevel::Error => LevelFilter::ERROR,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Trace => LevelFilter::TRACE,
        }
    }
}
impl Into<LevelFilter> for &LogLevel {
    fn into(self) -> LevelFilter {
        match self {
            LogLevel::Off => LevelFilter::OFF,
            LogLevel::Error => LevelFilter::ERROR,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Trace => LevelFilter::TRACE,
        }
    }
}
///
/// 
#[allow(dead_code)]
pub enum Backtrace {
    Full,
    Short,
}
impl ToString for Backtrace {
    fn to_string(&self) -> String {
        match self {
            Backtrace::Full => "full",
            Backtrace::Short => "short",
        }.to_owned()
    }
}
///
/// Call DebugSession::init() to initialize logging
#[allow(dead_code)]
pub struct DebugSession {
    level: LogLevel,
    modules: Vec<(String, LogLevel)>,
    backtrace: Backtrace,
}
///
/// 
impl DebugSession {
    ///
    /// Returns [DebugSession] new instance
    /// with default logging level `Debug` and backtrace `Short`
    /// 
    /// - Use `filter(...)` to customize logging filter
    /// - Use `module(...)` to customize logging filter for exact module
    /// 
    /// Example
    /// ```ignore
    /// DebugSession::new()
    ///     .filter(LogLevel::Info)
    ///     .module("my-module", LogLevel::Debug)
    ///     .init();
    /// ```
    pub fn new() -> Self {
        Self {
            level: LogLevel::Debug,
            modules: vec![],
            backtrace: Backtrace::Short,
        }
    }
    ///
    /// Default filtering lefel for all events
    pub fn filter(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }
    ///
    /// Filtering lefel for exact module
    pub fn module(mut self, module: impl Into<String>, level: LogLevel) -> Self {
        self.modules.push((module.into(), level));
        self
    }
    ///
    /// Initialize debug session on first call, all next will be ignored
    #[allow(dead_code)]
    pub fn init(self) {
        INIT.call_once(|| {
            let filter = Targets::new()
                .with_default(LevelFilter::DEBUG);
            let filter = self.modules.iter().fold(filter, |filter, (target, level)| {
                filter.with_target(target, level)
            });
            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer())
                .with(filter)
                .init();

            let backtrace = self.backtrace.to_string();
            env::set_var("RUST_BACKTRACE", &backtrace);
            assert_eq!(env::var("RUST_BACKTRACE"), Ok(backtrace.clone()), "Set env RUST_BACKTRACE={} failed", backtrace);
            // env::set_var("RUST_LOG_STYLE", log_style);     // auto / always / never
            // assert_eq!(env::var("RUST_LOG_STYLE"), Ok(log_style.to_string()), "Set env RUST_LOG_STYLE={} failed", log_style);
            match env_logger::builder().is_test(true).try_init() {
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
