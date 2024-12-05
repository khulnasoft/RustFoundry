impl TelemetryContext {
    // Use thread-local storage for fast access
    thread_local! {
        static CURRENT_SPAN: Cell<Option<SharedSpan>> = Cell::new(None);
    }
    
    pub fn current_span() -> Option<SharedSpan> {
        CURRENT_SPAN.with(|span| span.get())
    }
}
