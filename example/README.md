This example has original book in English and 2 translations: Russian and Spanish.

## Theme

`mdbook` can't receive to render custom fields and we must hardcode language list in template and
script.

Template copied from original but it has next differences:

1. Added [change language popup][lang-popup] (in top right corner, near print) by this snippet:

```html
<ul id="lang-list" class="theme-popup" style="left: auto;" aria-label="Languages" role="menu">
    <li role="none"><button role="menuitem" class="theme" id="en">English</button></li>
    <li role="none"><button role="menuitem" class="theme" id="ru">Русский</button></li>
    <li role="none"><button role="menuitem" class="theme" id="es">Espanol</button></li>
</ul>
```

[lang-popup]:https://github.com/funkill/mdbook-i18n/blob/master/example/theme/index.hbs#L126

This element uses theme styles (except overrided `left`)

2. Added [script][lang-script] for show/hide change language popup and redirect to selected translation
(modified script for change theme):

[lang-script]:https://github.com/funkill/mdbook-i18n/blob/master/example/theme/index.hbs#L282

```javascript
var langs = [
    'ru',
    'en',
    'es'
];

(function langs() {
    var html = document.querySelector('html');
    var langToggleButton = document.getElementById('lang-toggle');
    var langPopup = document.getElementById('lang-list');

    function showLangs() {
        langPopup.style.display = 'block';
        langToggleButton.setAttribute('aria-expanded', true);
    }

    function hideLangs() {
        langPopup.style.display = 'none';
        langToggleButton.setAttribute('aria-expanded', false);
        langToggleButton.focus();
    }

    langToggleButton.addEventListener('click', function () {
        if (langPopup.style.display === 'block') {
            hideLangs();
        } else {
            showLangs();
        }
    });

    langPopup.addEventListener('click', function (e) {
        var lang = e.target.id || e.target.parentElement.id;
        window.location.pathname = window.location.pathname
            .split('/')
            .map((s, idx) => {
                return idx == 2 ? lang : s;
            })
            .join('/');
    });

    langPopup.addEventListener('focusout', function(e) {
        // e.relatedTarget is null in Safari and Firefox on macOS (see workaround below)
        if (!!e.relatedTarget && !langToggleButton.contains(e.relatedTarget) && !langPopup.contains(e.relatedTarget)) {
            hideLangs();
        }
    });

    // Should not be needed, but it works around an issue on macOS & iOS: https://github.com/rust-lang-nursery/mdBook/issues/628
    document.addEventListener('click', function(e) {
        if (langPopup.style.display === 'block' && !langToggleButton.contains(e.target) && !langPopup.contains(e.target)) {
            hideLangs();
        }
    });
})();
```

This snippets injected into template (not in `addition-js` and separate file) because mdbook-i18n
currently before rendering not changed paths to addition files and all addition files must be presented
in every translation.

## Note

In this example parameter `multilingual` in `book.toml` has value `true`. `mdbook-i18n` don't use and don't check
this parameter. This parameter only checks by `mdbook` (and currently don't uses). This parameter not required.
