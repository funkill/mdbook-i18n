use std::{
    convert::TryFrom,
    error::Error as StdError,
    fmt::{Error as FmtError, Formatter},
    path::PathBuf,
    result::Result as StdResult,
};
use mdbook::{
    config::{BookConfig, BuildConfig, Config as MdBookConfig},
    renderer::RenderContext,
};
use toml::Value;
use toml::value::Table;

const BASE_OUT_DIR: &str = "i18n";

#[derive(Debug)]
pub(crate) struct RenderConfig(pub(crate) Vec<RenderItem>);

impl TryFrom<RenderContext> for RenderConfig {
    type Error = TryFromRenderContext;

    fn try_from(context: RenderContext) -> StdResult<Self, Self::Error> {
        let mut config = context.config;
        let build_config = config.build.clone();
        let root = context.root.clone();

        let output = config.get_mut("output.html").unwrap_or(&mut Value::Table(Table::default())).clone();
        let mut books = config
            .get_mut("output.i18n.translations")
            .and_then(|value| value.as_array())
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|value| value.as_table().cloned())
            .filter(|option| option.is_some())
            .map(Option::unwrap)
            .map(|table| {
                let language = String::from(
                    table
                        .get("language")
                        .expect("Not found for one of translations")
                        .as_str()
                        .expect("Language for one of translations not a string"),
                );
                let book = {
                    let mut book: BookConfig =
                        Value::Table(table).try_into().expect("Can't parse config");
                    if book.src.as_os_str() == "src" {
                        book.src = PathBuf::from("translations");
                        book.src.push(language.clone());
                    }

                    book
                };

                RenderItem::from(book, build_config.clone(), root.clone(), output.clone(), language)
            })
            .collect::<Vec<_>>();

        let language = config
            .book
            .language
            .clone()
            .expect("Language for main book not found");

        let main_book = RenderItem::from(config.book, config.build, root, output, language);

        books.insert(0, main_book);

        Ok(RenderConfig(books))
    }
}

#[derive(Debug, Clone)]
pub struct TryFromRenderContext(());

impl StdError for TryFromRenderContext {}

impl std::fmt::Display for TryFromRenderContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdResult<(), FmtError> {
        f.write_str("TryFromRenderContext")
    }
}

#[derive(Debug)]
pub(crate) struct RenderItem {
    pub(crate) mdbook_config: MdBookConfig,
    pub(crate) root: PathBuf,
    pub(crate) language: String,
}

impl RenderItem {
    pub fn from(
        book: BookConfig,
        build: BuildConfig,
        root: PathBuf,
        rest: Value,
        language: String,
    ) -> RenderItem {
        let mut build = build;
        fn set_build_path(build: &mut BuildConfig, language: &str) {
            build.build_dir.push(BASE_OUT_DIR);
            build.build_dir.push(language);
        }

        fn mdbook_from_configs(book: BookConfig, build: BuildConfig, rest: Value) -> MdBookConfig {
            let mut new_config = MdBookConfig::default();
            new_config.book = book;
            new_config.build = build;
            new_config.set("output.html", rest.clone()).unwrap();
            new_config
        }

        set_build_path(&mut build, &language);
        let config = mdbook_from_configs(book, build, rest);

        RenderItem {
            mdbook_config: config,
            root,
            language,
        }
    }
}
