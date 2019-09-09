mod config;

use crate::config::Config;
use failure::{Error, SyncFailure};
use mdbook::{renderer::RenderContext, MDBook};
use std::{convert::TryInto, io};

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    init_logger();

    log::info!("Running mdbook-i18n");
    let config = get_config()?;

    log::debug!("Prepare config");
    let prepared_configs: Vec<crate::config::PreparedConfig> = config.into();
    for config in prepared_configs {
        log::info!("Build book for language {}", config.lang);
        MDBook::load_with_config(config.root, config.mdbook_config)
            .and_then(|mdbook| mdbook.build())
            .map_err(error_from_unsync)?;
    }

    Ok(())
}

fn get_config() -> Result<Config> {
    log::debug!("Getting config");
    let mut stdin = io::stdin();

    RenderContext::from_json(&mut stdin)
        .map_err(error_from_unsync)
        .and_then(|cfg| TryInto::try_into(cfg).map_err(Into::into))
}

fn error_from_unsync<E: std::error::Error + Send + 'static>(e: E) -> Error {
    SyncFailure::new(e).into()
}

// Copied from mdbook
fn init_logger() {
    use env_logger::Builder;
    use std::{env, io::Write};
    use log::LevelFilter;
    use chrono::Local;

    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
    }

    builder.init();
}
