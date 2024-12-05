impl TelemetryContext {
    // Use thread-local storage for fast access
    thread_local! {
        static CURRENT_SPAN: Cell<Option<SharedSpan>> = Cell::new(None);
    }
    
    pub fn current_span() -> Option<SharedSpan> {
        CURRENT_SPAN.with(|span| span.get())
    }

    pub fn with_error_context<T, E>(
        &self,
        operation: impl FnOnce() -> Result<T, E>,
        context: &str
    ) -> Result<T, TelemetryError> 
    where
        E: std::error::Error
    {
        operation().map_err(|e| {
            log::error!("{}: {}", context, e);
            TelemetryError::TracingInit(format!("{}: {}", context, e))
        })
    }
}
