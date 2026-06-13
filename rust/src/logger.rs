use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init(log_level: Option<&str>) {
    let filter = if let Some(level) = log_level {
        EnvFilter::new(level)
    } else {
        EnvFilter::from_default_env()
            .add_directive("webdav_backup=info".parse().unwrap())
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(false)
                .with_level(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .compact(),
        )
        .init();
}
