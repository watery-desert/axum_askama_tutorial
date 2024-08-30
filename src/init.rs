use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logging");
}
