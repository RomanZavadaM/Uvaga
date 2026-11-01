# TEST_PLAN — критерії до коду

## Етап 0: project continuity/docs
Наявні START, RULES, STATE, WORKLOG, HANDOFF, REQUIREMENTS, ARCHITECTURE, SOURCES, LICENSE, COPYRIGHT, THIRD_PARTY_NOTICES. Лінки несуперечливі, секретів/API token немає, статус не видає недоведене за перевірене. Чиста нова сесія має знайти наступну дію без попереднього чату й без обов'язкового відкриття central dev-strategy.

## Core regression
Обов'язково перевіряти:
1. verified official start/clear;
2. unverified clear rejected;
3. old/equal event rejected conservatively;
4. reused provider record start→clear produces distinct event-version identity;
5. duplicate event does not multiply/change state;
6. source stale/unavailable never creates clear;
7. stale previous clear is not presented as current safe state;
8. aggregator data cannot self-promote to official state;
9. unknown region/territory fails closed;
10. TTL supplemental item does not clear official alert.

## Docs/continuity verification
- START/HANDOFF дають одну наступну дію.
- REQUIREMENTS/ARCHITECTURE/SOURCE_VALIDATION не містять правила «TTL = відбій».
- License не перевизначає third-party licenses.
- Drive mirror/checkpoint не називається повним Git backup.
- Live PR/head/CI звіряються перед інтеграцією.
- Результат перевірки фіксує дату, SHA/run, дефекти та PASS/FAIL для конкретного scope.

## Перед будь-яким production
Окремо перевірити API terms/schema, reliability/load, secrets, privacy, background push Android/iOS, accessibility, rollback, backup/restore, full dependency-license inventory та живе приймання. CI не замінює production acceptance.