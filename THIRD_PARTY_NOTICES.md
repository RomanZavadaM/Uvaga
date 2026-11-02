# Third-party notices — ТривогА / Uvaga

ТривогА є власницьким програмним забезпеченням, але використовує сторонні open-source компоненти, які залишаються під власними ліцензіями.

## Поточний технічний стан

Початкове твердження про «docs-only» більше не актуальне: прототип уже містить Rust/Tauri 2 application shell та транзитивні Rust-залежності.

Для r18 exact locked graph виконано відтворюваний inventory:
- GitHub Actions run: `41422471078`;
- artifact: `13130467223` (`Uvaga-r18-dependency-license-inventory`);
- artifact ZIP SHA256: `afc0c0a75986fc68ea836a2cf7778353735dfdcb8bb24e5b99368e23ca351f7f`;
- перевірений `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- 408 resolved packages: 2 локальні proprietary + 406 crates.io;
- 0 зовнішніх git/non-registry dependencies;
- 0 пакетів без заявленого Cargo license expression.

Повний машинний inventory і аналіз: [docs/DEPENDENCY_LICENSE_REVIEW.md](docs/DEPENDENCY_LICENSE_REVIEW.md).

## Виявлені ліцензійні сімейства

У поточному locked graph зустрічаються MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-3-Clause, ISC, Zlib, Unicode-3.0, Unlicense, 0BSD, MIT-0, CC0-1.0 та MPL-2.0, а також комбіновані `OR`/`AND` expressions на їх основі.

Окремі пакети `r-efi` мають expression `MIT OR Apache-2.0 OR LGPL-2.1-or-later`; поточний graph не містить LGPL-only/GPL-only/AGPL-only package expression.

## MPL-2.0 компоненти

П'ять транзитивних пакетів у поточному lock мають MPL-2.0 без альтернативної ліцензії:

- `cssparser 0.37.0` — https://github.com/servo/rust-cssparser
- `cssparser-macros 0.7.1` — https://github.com/servo/rust-cssparser
- `dtoa-short 0.3.5` — https://github.com/upsuper/dtoa-short
- `option-ext 0.2.0` — https://github.com/soc/option-ext
- `selectors 0.38.0` — https://github.com/servo/stylo

MPL 2.0 допускає включення MPL-коду до більшої proprietary роботи, але при зовнішньому розповсюдженні executable потрібно виконувати вимоги MPL щодо Covered Software, зокрема повідомити одержувача, як отримати відповідний Source Code Form. Офіційні матеріали:
- https://www.mozilla.org/MPL/2.0/
- https://www.mozilla.org/MPL/2.0/FAQ/

Uvaga не заявляє авторських прав на ці компоненти та не перепризначає їхні ліцензії.

## До production release

Перед публічним production-релізом потрібно окремо завершити:

1. machine-enforced dependency/license/source policy;
2. генерацію та перевірку повного набору сторонніх license/notice texts для exact release lock;
3. source-availability notice для exact MPL-covered versions;
4. повторний аудит після будь-якої зміни dependency lock.

Machine-enforced license/source policy вже перевірена: GitHub Actions run `41423140592` — **PASS** (`cargo deny check licenses sources`).

До production release ще не закритий саме packaging-підетап: повний набір сторонніх license/notice texts та exact MPL source-availability notice для фінального release lock.
