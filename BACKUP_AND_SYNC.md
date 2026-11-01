# BACKUP_AND_SYNC — ТривогА

GitHub — авторитетний стан коду, документації, рішень, журналу та PR. Google Drive — одностороння копія з GitHub; редагування дзеркала не переписує GitHub.

## Що зберігаємо
- Git-історію, теги, прийняту документацію, відкриті PR і контрольні пакети.
- Незлиту унікальну роботу — захищений checkpoint у GitHub-гілці/PR або окремий датований архів, незалежний від робочої сесії.
- Release assets, налаштування сервісу та користувацькі дані — після появи; секрети окремо, не в публічному GitHub/Drive.
- Тимчасові оперативні події за замовчуванням НЕ архівуємо без визначеної мети; аварійний state snapshot може бути потрібний для відновлення, але TTL додаткової інформації не скасовує активну офіційну тривогу.

## Точки збереження
Після значущого checkpoint, PR merge і релізу — датований знімок; записати базовий SHA, дату, контрольну суму/ідентифікатор, місце, результат завантаження. Дзеркало PROJECT зберігає актуальні START/RULES/STATE/HANDOFF/WORKLOG; CHECKPOINTS — контрольні покажчики; BACKUPS — окремі датовані знімки; VERIFICATION — перевірки читання/відновлення.

## Google Drive mirror — перевірено 2026-11-01
Папка: https://drive.google.com/drive/folders/1sPq7Ajmi4uhydjK7Wy1B-vR5IwTFVt8b

У PROJECT створено й повторно прочитано:
- START_HERE.md
- PROJECT_RULES.md
- PROJECT_STATE.md
- WORKLOG.md
- HANDOFF_CURRENT.md

Кожне дзеркало містить GitHub blob SHA. Повторне читання з Drive підтвердило збіг нормалізованого тексту з GitHub для всіх 5 документів. Google Docs може згортати порожні Markdown-абзаци, тому це змістове текстове дзеркало, не byte-for-byte raw Markdown archive.

Доказ: https://docs.google.com/document/d/1W7cGX-l0ozK7Se8Q0OEN8WM-WqanVBwv-V53JRqAlgA/edit

## Незакритий backup/restore
Google Drive mirror НЕ є незалежною резервною копією Git-історії. Спроба створити повний Git bundle у контейнері 2026-11-01 заблокована мережевим середовищем: контейнер не міг resolve github.com. Отже:
- mirror write/readback — PASS;
- full Git-history bundle — NOT DONE;
- restore Git refs/tags/branches from independent archive — NOT DONE.

До першого реального розгортання виконати повний незалежний Git backup + test restore у середовищі з GitHub-доступом. Не називати C12 повністю виконаним до цього доказу.
