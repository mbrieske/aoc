use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn tracing_init() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .without_time()
        .with_target(false)
        .with_span_events(FmtSpan::ENTER)
        .finish();
    tracing::subscriber::set_global_default(subscriber).ok();
}
