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

const bridge = document.querySelector("[data-bridge]");
const region = document.querySelector("[data-region]");
const regionName = document.querySelector("[data-region-name]");
const confirmed = document.querySelector("[data-confirmed]");
const freshness = document.querySelector("[data-freshness]");
const freshnessDetail = document.querySelector("[data-freshness-detail]");
const presentation = document.querySelector("[data-presentation]");
const issued = document.querySelector("[data-issued]");
const raw = document.querySelector("[data-raw]");
const status = document.querySelector("[data-status]");
const refresh = document.querySelector("[data-refresh]");
const territory = document.querySelector("[data-territory]");
const scenario = document.querySelector("[data-scenario]");
const versionNodes = document.querySelectorAll("[data-app-version]");

const allowedPresentationKinds = new Set([
  "active_current",
  "active_last_known",
  "clear_needs_refresh",
  "cleared_current",
  "unknown",
  "error",
]);

function label(group, value) {
  return labels[group]?.[value] ?? value ?? "—";
}

function setControlsDisabled(disabled) {
  refresh.disabled = disabled;
  territory.disabled = disabled;
  scenario.disabled = disabled;
  bridge.setAttribute("aria-busy", String(disabled));
}

function setPresentationKind(kind) {
  bridge.dataset.presentation = allowedPresentationKinds.has(kind) ? kind : "unknown";
}

function setVersion(value) {
  const safe = typeof value === "string" && value.trim() ? value.trim() : "—";
  versionNodes.forEach((node) => {
    node.textContent = safe;
  });
}

function clearState() {
  region.textContent = "—";
  regionName.textContent = "Синтетична територія";
  confirmed.textContent = "Не визначено";
  freshness.textContent = "Стан джерела невідомий";
  freshnessDetail.textContent = "Не визначено";
  presentation.textContent = "Недостатньо даних";
  issued.textContent = "—";
  raw.textContent = "Синтетичний стан не отримано.";
}

function setError(message) {
  status.textContent = message;
  status.dataset.kind = "error";
  clearState();
  setPresentationKind("error");
}

function renderSnapshot(data) {
  region.textContent = data.region;
  regionName.textContent = data.territory_name;
  confirmed.textContent = label("confirmed_state", data.confirmed_state);
  freshness.textContent = label("source_freshness", data.source_freshness);
  freshnessDetail.textContent = label("source_freshness", data.source_freshness);
  presentation.textContent = label("presentation", data.presentation);
  issued.textContent = data.issued_at ? String(data.issued_at) : "немає події";
  raw.textContent = `synthetic=true · app=${data.app_version ?? "—"} · scenario=${data.scenario} · issued_at=${data.issued_at}`;
  setPresentationKind(data.presentation);
  setVersion(data.app_version);
}

async function loadDemo() {
  status.textContent = "Отримую синтетичний стан із Rust core…";
  status.dataset.kind = "loading";
  setControlsDisabled(true);

  try {
    const invoke = window.__TAURI__?.core?.invoke;
    if (!invoke) {
      throw new Error("Tauri IPC недоступний");
    }

    const payload = await invoke("demo_snapshot", {
      territory: territory.value,
      scenario: scenario.value,
    });
    const data = JSON.parse(payload);

    renderSnapshot(data);
    status.textContent = "Стан отримано локально через Tauri IPC.";
    status.dataset.kind = "ok";
  } catch (error) {
    console.error(error);
    setError("Не вдалося отримати синтетичний стан. Це не означає відбій.");
  } finally {
    setControlsDisabled(false);
  }
}

refresh.addEventListener("click", loadDemo);
territory.addEventListener("change", loadDemo);
scenario.addEventListener("change", loadDemo);
window.addEventListener("DOMContentLoaded", loadDemo);
