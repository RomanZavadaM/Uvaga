# r24 — release evidence wording consistency

## Мета

Усунути застаріле формулювання в локалізованих README без переписування історичного evidence.

## Predeclared acceptance

1. Український README лишається основним; EN/FR/DE/ES/KO/JA передають той самий release-status зміст.
2. r20 описаний як unsigned archived installer test candidate з automated install/launch/uninstall evidence; owner installer/visual acceptance лишається PENDING.
3. r19 описаний як historical synthetic desktop-hardening evidence, а не як "latest" build.
4. Новіші CI/docs/release-process hardening кроки не трактуються як новий owner-accepted або public release artifact.
5. Історичні artifact URLs, source/merge SHA та SHA256 r19/r20 не змінюються.
6. App version лишається `0.1.0`; цей docs-only етап не є version bump.
7. No product/safety logic, Cargo.lock, workflow trigger, live source, signing, public release or production deployment change.

## Boundary

r24 виправляє лише документаційний статус evidence. Він не створює нового binary candidate і не змінює acceptance рівень існуючих artifact.
