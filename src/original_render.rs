use crate::config::RenderConfig;
use mdbook::MDBook;

pub(crate) struct OriginalRender;

impl OriginalRender {
    pub(crate) fn render(config: RenderConfig) -> anyhow::Result<()> {
        for item in config.0 {
            log::info!("Build for language {}", item.language);
            MDBook::load_with_config(item.root, item.mdbook_config)
                .and_then(|mdbook| mdbook.build())?;
        }

        Ok(())
    }
}
