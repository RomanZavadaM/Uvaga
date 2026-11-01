# ARCHITECTURE — попередня пропозиція
Rust-ядро: джерела → adapter ingest → нормалізація → provenance/verify → dedup/TTL → регіональний стан → інтерфейси.
Tauri 2 — кандидат оболонки Windows 10/11 x64, macOS x64/ARM64, Android, iOS. Веб — окремий клієнт зі спільним контрактом даних. Desktop підтримувати installer/portable після технічної перевірки. Telegram — окремий майбутній вихід, не початковий бот.
Сховище: оперативна пам'ять + за потреби невеликі відновлювані snapshot/SQLite; PostgreSQL не планується. Короткий TTL додаткової інформації не може закрити активну офіційну тривогу. Невідомий стан і стан втрати зв'язку окремі.
Протокол JSON: source_id, event_id, verified_status, region_id, event_type, issued_at, observed_at, expires_at (для додаткових повідомлень), provenance. Без вигадування точності даних.
Перед визначенням реалізації: аудит API/ліцензій, фонового режиму Android/iOS, способу доставки критичних сповіщень, offline UX, навантаження і безпечності.

## Public relay boundary
Публічні desktop/mobile/web клієнти не зберігають third-party API tokens. Tokened source adapters працюють у легкому контрольованому Rust relay/cache. Relay нормалізує події, дотримується rate limits, використовує conditional requests де дозволено, відстежує source health і видає клієнтам лише наш нормалізований контракт. Основний оперативний стан може жити в RAM; за потреби crash-recovery snapshot/SQLite. PostgreSQL не є вимогою.

Relay не перетворює timeout, 401/403/429, порожню відповідь чи stale cache на «відбій». Секрети не входять у GitHub, клієнтські пакети чи Google Drive mirror.
