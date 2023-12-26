use indicatif::{ProgressBar, ProgressStyle};
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

pub fn progressbar_init(total_iterations: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(total_iterations);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    progress_bar
}
