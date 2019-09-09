use failure::Fail;
use mdbook::{
    config::BookConfig as MdbookBookConfig, config::BuildConfig, config::Config as MdbookConfig,
    renderer::RenderContext,
};
use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
};
use toml::{value::Table, Value};

#[derive(Debug)]
pub struct Config {
    version: String,
    root: PathBuf,
    translations: Vec<BookConfig>,
    build: BuildConfig,
    destination: PathBuf,
}

#[derive(Debug)]
pub struct BookConfig {
    title: String,
    authors: Vec<String>,
    translators: Option<Vec<String>>,
    description: Option<String>,
    src: PathBuf,
    language: String,
    fallback: bool,
}

impl TryFrom<MdbookBookConfig> for BookConfig {
    type Error = MdbookBookConfig;

    fn try_from(book: MdbookBookConfig) -> Result<BookConfig, Self::Error> {
        if let (Some(title), Some(language)) = (book.title.clone(), book.language.clone()) {
            Ok(BookConfig {
                title,
                authors: book.authors,
                translators: None,
                description: book.description,
                src: book.src,
                language,
                // In Mdbook config BookConfig represents only main book, other books describes in plugin config
                fallback: true,
            })
        } else {
            Err(book)
        }
    }
}

impl TryFrom<Table> for BookConfig {
    type Error = Table;

    fn try_from(table: Table) -> Result<BookConfig, Self::Error> {
        let mut table = table;
        let authors = if let Some(Value::Array(authors)) = table.remove("authors") {
            authors
                .into_iter()
                .filter(Value::is_str)
                .map(|author| author.as_str().unwrap().to_string())
                .collect()
        } else {
            vec![]
        };

        let title = if let Some(Value::String(title)) = table.remove("title") {
            title
        } else {
            return Err(table);
        };

        let description = if let Some(Value::String(description)) = table.remove("description") {
            Some(description)
        } else {
            None
        };

        let language = if let Some(Value::String(language)) = table.remove("language") {
            language
        } else {
            return Err(table);
        };

        let translators = if let Some(Value::Array(translations)) = table.remove("translators") {
            Some(
                translations
                    .into_iter()
                    .filter(Value::is_str)
                    .map(|translation| translation.as_str().unwrap().to_string())
                    .collect(),
            )
        } else {
            None
        };

        let src = match table.remove("src") {
            Some(Value::String(src)) => src.into(),
            val @ Some(_) | val @ None => {
                if val.is_some() {
                    log::warn!("Source dir not a string");
                }

                let mut path = PathBuf::from("translations");
                path.push(&language);
                path
            }
        };

        Ok(BookConfig {
            authors,
            title,
            description,
            translators,
            language,
            src,
            fallback: false,
        })
    }
}

impl From<BookConfig> for MdbookBookConfig {
    fn from(config: BookConfig) -> MdbookBookConfig {
        MdbookBookConfig {
            authors: config.authors,
            description: config.description,
            language: Some(config.language),
            multilingual: false,
            src: config.src,
            title: Some(config.title),
        }
    }
}
#[derive(Debug, Fail)]
pub enum TryFromRenderContext {
    #[fail(display = "Trying convert render context")]
    Render(RenderContext),
    #[fail(display = "Trying convert mdbook config")]
    Book(MdbookBookConfig),
}

impl From<MdbookBookConfig> for TryFromRenderContext {
    fn from(cfg: MdbookBookConfig) -> TryFromRenderContext {
        TryFromRenderContext::Book(cfg)
    }
}

impl From<RenderContext> for TryFromRenderContext {
    fn from(cfg: RenderContext) -> TryFromRenderContext {
        TryFromRenderContext::Render(cfg)
    }
}

impl TryFrom<RenderContext> for Config {
    type Error = TryFromRenderContext;

    fn try_from(context: RenderContext) -> Result<Config, Self::Error> {
        let mut context = context;
        let books = {
            let book: BookConfig = context.config.book.clone().try_into()?;
            let mut books = match context.config.get_mut("output.i18n.translations") {
                Some(Value::Array(i18_config)) => {
                    let mut translations = vec![];
                    for translation in i18_config {
                        if !translation.is_table() {
                            log::warn!(
                                "Translation config not represented as TOML table: {:?}",
                                translation
                            );
                            continue;
                        }

                        let translation = translation.as_table().unwrap().clone();
                        if let Ok(translation) = translation.try_into() {
                            translations.push(translation);
                        } else {
                            log::warn!("Translation config can not converted.");
                            continue;
                        }
                    }

                    translations
                }
                None => vec![],
                _ => return Err(context.into()),
            };

            books.push(book);
            books
        };

        Ok(Config {
            build: context.config.build,
            destination: context.destination,
            root: context.root,
            translations: books,
            version: String::from("0.0.0"),
        })
    }
}

#[derive(Debug)]
pub(crate) struct PreparedConfig {
    pub(crate) mdbook_config: MdbookConfig,
    pub(crate) root: PathBuf,
    pub(crate) lang: String,
}

impl From<Config> for Vec<PreparedConfig> {
    fn from(config: Config) -> Vec<PreparedConfig> {
        let build_config = config.build.clone();
        let root = config.root.clone();

        config
            .translations
            .into_iter()
            .map(|translation| {
                let mut build = build_config.clone();
                let language = translation.language.clone();
                build.build_dir.push(&language);

                let mut cfg = MdbookConfig::default();
                cfg.book = translation.into();
                cfg.build = build;
                (cfg, language)
            })
            .map(|(config, lang)| PreparedConfig {
                mdbook_config: config,
                root: root.clone(),
                lang,
            })
            .collect()
    }
}
