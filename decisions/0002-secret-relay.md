# ADR-0002: Секретні API ключі лише на контрольованому relay

Дата: 2026-11-01
Статус: прийнято як архітектурне правило до перегляду після договорів з постачальниками API.

## Контекст
Публічні desktop/mobile/web клієнти не можуть безпечно містити секретні third-party API tokens. Документація alerts.in.ua прямо вимагає для public services використовувати власний server proxy та не хардкодити token у клієнті.

## Рішення
- Увести легкий проектний relay/cache між tokened source API та публічними клієнтами.
- Relay тримає секрети поза репозиторієм і клієнтськими пакетами, контролює rate limits, conditional requests, source health і нормалізацію.
- PostgreSQL не є вимогою. Поточний стан — RAM; за потреби короткий crash-recovery snapshot/SQLite.
- Relay не має права створити «clear» через timeout, HTTP error чи порожню відповідь.
- Telegram у майбутньому може споживати той самий нормалізований вихід relay або інтеграційний feed.

## Не вирішує
Цей ADR не затверджує конкретного постачальника API, hosting, push transport чи production deployment.