#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    #[error("Tracing initialization failed: {0}")]
    TracingInit(String),
    
    #[error("Logger nesting exceeded maximum depth of {0}")]
    LoggerNestingTooDeep(usize),
    
    #[error("Span context serialization failed: {0}")]
    SpanSerialization(String),
    
    #[error("Memory profiler error: {0}")]
    MemoryProfiler(String)
}
