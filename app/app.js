const labels = {
  confirmed_state: {
    active: "Тривога активна",
    cleared: "Підтверджений відбій",
    unknown: "Офіційний стан невідомий",
  },
  source_freshness: {
    fresh: "Джерело актуальне",
    stale: "Дані застаріли",
    unavailable: "Джерело недоступне",
    unknown: "Стан джерела невідомий",
  },
  presentation: {
    active_current: "Активна — актуальні дані",
    active_last_known: "Активна — останній підтверджений стан",
    clear_needs_refresh: "Відбій потребує повторного підтвердження",
    cleared_current: "Відбій підтверджений актуальним джерелом",
    unknown: "Недостатньо даних",
  },
};

const region = document.querySelector("[data-region]");
const confirmed = document.querySelector("[data-confirmed]");
const freshness = document.querySelector("[data-freshness]");
const presentation = document.querySelector("[data-presentation]");
const raw = document.querySelector("[data-raw]");
const status = document.querySelector("[data-status]");
const refresh = document.querySelector("[data-refresh]");

function label(group, value) {
  return labels[group]?.[value] ?? value ?? "—";
}

function setError(message) {
  status.textContent = message;
  status.dataset.kind = "error";
  confirmed.textContent = "Не визначено";
  freshness.textContent = "Не визначено";
  presentation.textContent = "Не визначено";
  raw.textContent = "Синтетичний стан не отримано.";
}

async function loadDemo() {
  status.textContent = "Отримую синтетичний стан із Rust core…";
  status.dataset.kind = "loading";
  refresh.disabled = true;

  try {
    const invoke = window.__TAURI__?.core?.invoke;
    if (!invoke) {
      throw new Error("Tauri IPC недоступний");
    }

    const payload = await invoke("demo_snapshot");
    const data = JSON.parse(payload);

    region.textContent = data.region;
    confirmed.textContent = label("confirmed_state", data.confirmed_state);
    freshness.textContent = label("source_freshness", data.source_freshness);
    presentation.textContent = label("presentation", data.presentation);
    raw.textContent = `synthetic=true · issued_at=${data.issued_at}`;

    status.textContent = "Стан отримано локально через Tauri IPC.";
    status.dataset.kind = "ok";
  } catch (error) {
    console.error(error);
    setError("Не вдалося отримати синтетичний стан. Це не означає відбій.");
  } finally {
    refresh.disabled = false;
  }
}

refresh.addEventListener("click", loadDemo);
window.addEventListener("DOMContentLoaded", loadDemo);
