# mdBook-i18n

Simple mdBook i18n plugin.

## Installation

It requires mdbook >= 0.3.1.

```sh
# cargo install mdbook
# cargo install mdbook-i18n
```

## Usage

1. Add `language` to `book` section in your `book.toml`.
2. Add `output.i18n.translations` table to your `book.toml`. Every record in this table
must contains `language` and `title`. Also records can contains fields `authors` (must be array),
`translators` (also must be array), `description` and `src`. If `src` not present in record, then
this field creates as `<book's root>/translations/<language name>`.
3. Write translations.
4. Run `mdbook build` for build all books. Every book saves in destination directory in folder with
locale name.

## How it works?

Source book from config converts to translation config. Common configs shares between all
translations. After that `mdbook` runs for every translation.

## Limitations

1. Custom values from main config don't send to mdbook config. For now this project used native
`RenderContext` what have private `rest` field in `config` (this field contains custom values
from config).
2. Books don't share assets. Because native render of mdbook can build only one build, every build
generate full tree of assets.
3. Books don't has links to different l10n. Because used native render without custom templates.
4. Maybe everything else what i forget.

## License

[MIT](LICENSE)
