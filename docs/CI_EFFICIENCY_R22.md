# r22 — CI efficiency and release checklist

## Мета

Зменшити марні дорогі GitHub Actions runs без послаблення product/safety/release gates і зафіксувати єдиний фінальний release checklist.

## Predeclared acceptance

1. Docs-only зміни не запускають Windows synthetic artifact або NSIS installer build автоматично.
2. Product/UI/Tauri/config зміни як і раніше запускають відповідні Windows artifact/installer gates.
3. Cheap docs/UI/release-metadata checks лишаються автоматичними для своїх областей.
4. Heavy installer workflow лишається доступним через `workflow_dispatch` для фінального candidate з exact ref.
5. Release checklist вимагає фінальний installer candidate **після** завершення release docs/version metadata, а не використовує старий binary artifact як production evidence.
6. Release checklist вимагає exact source SHA, CI PASS, artifact readback SHA256, dependency/license compliance, owner manual install/visual acceptance, signing decision і non-expiring license/MPL source availability.
7. Release checklist прямо забороняє public release/live-source integration як автоматичний наслідок CI PASS.
8. CI-scope guard автоматично падає, якщо broad docs paths знову випадково додадуться до heavy Windows workflows.
9. Existing r17–r21 evidence/artifact IDs/hashes не змінюються.
10. No product logic, safety mapping, live API, signing, updater or version bump in r22.

## Efficiency boundary

Мета — прибрати **повторну компіляцію**, а не перевірки. Документаційні зміни продовжують проходити docs/UI checks. Коли документація входить до фінального distribution artifact, фінальний installer candidate генерується вручну з exact release ref після замороження release docs.
