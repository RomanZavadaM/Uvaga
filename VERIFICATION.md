# VERIFICATION — ТривогА

**Оновлено:** 2026-11-01  
Обсяг: organizational checkpoint + offline/synthetic prototype slices.

## GitHub
- Public repo: `RomanZavadaM/Uvaga`.
- Initial main: `ad8653ebcdf16507be54a929b690c8a1060e035b`.
- Verification/integration порядок: Issue #9.
- Один активний виконавець за замовчуванням; паралельність — лише за рішенням власника.

## Перевірений технічний стан
- Docs/core/relay/parser/freshness/recovery/presentation/territory slices мають успішні GitHub CI checkpoints.
- Tauri product-platform checks проходили на Windows x64, macOS ARM64 та macOS Intel.
- Реальні CI failures на required Tauri icon/build assets були виявлені й виправлені до зелених runs.
- Current head кожного PR потрібно повторно звіряти перед merge; старі run IDs не є безстроковим PASS.

## Source/document audit
UkraineAlarm public access/offer та alerts.in.ua docs досліджено без credentials/live calls. KATOTTG production policy вимагає Ministry base + applicable amendments.
No production/public alerting performed.

## License
Project proprietary. Direct dependency license audit існує; full transitive dependency/license inventory до release ще відкритий.

## Drive
PROJECT mirror записано і readback-перевірено.
Текстовий snapshot із blob SHA markers існує, але native Google Docs нормалізує представлення тексту, тому byte-exact restore не доведений.
Full raw/git backup + restore: **NOT DONE**.

## Не завершено
- Base-first merge/retarget sequence to main.
- Android/iOS build validation.
- Live API credentials/schema/latency/webhook/security/legal acceptance.
- Public notification/background push.
- Full raw/git backup/restore.
- Full transitive license gate.
- Release artifacts/signing/installers.
- Формальна clean-session continuity перевірка C13.

Тести/CI, виконані в межах поточного робочого процесу, є project verification evidence. Окремий reviewer не припускається автоматично; додаткову перевірку може призначити власник.