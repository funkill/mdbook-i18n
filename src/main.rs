mod config;

use crate::config::Config;
use failure::{Error, SyncFailure};
use mdbook::{renderer::RenderContext, MDBook};
use std::{convert::TryInto, io};

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let config = get_config()?;
    let prepared_configs: Vec<crate::config::PreparedConfig> = config.into();
    for i in prepared_configs {
        MDBook::load_with_config(i.root, i.mdbook_config)
            .and_then(|mdbook| mdbook.build())
            .map_err(error_from_unsync)?;
    }

    Ok(())
}

fn get_config() -> Result<Config> {
    let mut stdin = io::stdin();

    RenderContext::from_json(&mut stdin)
        .map_err(error_from_unsync)
        .and_then(|cfg| TryInto::try_into(cfg).map_err(Into::into))
}

fn error_from_unsync<E: std::error::Error + Send + 'static>(e: E) -> Error {
    SyncFailure::new(e).into()
}
