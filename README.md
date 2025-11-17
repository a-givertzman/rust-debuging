# Tools useful for debuggung rust application

## DebugSession

DebugSession::new() - returns `DebugSession` new instance
with default logging level `Debug` and backtrace `Short`

- Use `filter(...)` to customize logging filter
- Use `module(...)` to customize logging filter for exact module

Example
```ignore
DebugSession::new()
    .filter(LogLevel::Info)
    .module("my-module", LogLevel::Debug)
    .init();
```