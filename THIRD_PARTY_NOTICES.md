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

## Поточний compliance status і production gate

Machine-enforced license/source policy перевірена: GitHub Actions run `41423140592` — **PASS** (`cargo deny check licenses sources`).

Для exact Windows normal+build dependency closure packaging-доказ закритий: PR #41 / run `41426326573` — **PASS**, 222/222 зовнішніх пакетів, missing 0, extra 0; artifact `13131534490`, ZIP SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e`.

Broad exact-lock compliance bundle PR #40 також збережений: 406 crates indexed, 371 package-local notice/license captures, 35 без package-local файлів явно зафіксовані, а повні source trees усіх 5 MPL-2.0 пакетів збережені в `MPL-SOURCE-r18.tar.gz`.

r19/r20 desktop build gate відтворює той самий accepted Cargo.lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`. Тому exact Windows license texts можуть повторно використовуватись лише доки цей SHA збігається; будь-який dependency drift блокує build до нового аудиту.

Перед **production distribution** ще потрібні:
1. stable non-expiring розміщення full third-party license texts і exact MPL-covered source material;
2. code signing / publisher provenance;
3. final artifact readback і owner acceptance;
4. повторний аудит після будь-якої зміни dependency lock.

Майбутні macOS/Android/iOS релізи потребують власного target-specific completeness check на exact final release lock.

## Compliance bundle checkpoint

GitHub Actions run `41423995739` produced and read back artifact `13131470682` (`Uvaga-r18-third-party-compliance-bundle`), ZIP SHA256 `b91319651fe2ffa9d7983195fcc6e82bee77ed8357d36459aedb8a83c2a7d160`.

- 406 crates.io packages indexed;
- 371 have captured package-local license/notice files;
- 35 have no package-local license/notice file and require supplemental license text/attribution;
- complete vendored source for the five exact MPL-2.0 packages is preserved in `MPL-SOURCE-r18.tar.gz` inside the artifact.

Broad package-local-file collection саме по собі було **PASS/PARTIAL**, але окремий Windows cargo-about completeness gate пізніше закрив повні license texts як **PASS 222/222**.
