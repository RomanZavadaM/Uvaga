use uvaga_core::{
    presentation::{presentation_state, PresentationState},
    AlertState, OfficialEvent, RegionStatus, Signal, SourceFreshness,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DemoTerritory {
    code: &'static str,
    name_uk: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DemoScenario {
    ActiveFresh,
    ActiveStale,
    ClearedFresh,
    ClearedStale,
    UnknownUnavailable,
}

impl DemoScenario {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "active_fresh" => Some(Self::ActiveFresh),
            "active_stale" => Some(Self::ActiveStale),
            "cleared_fresh" => Some(Self::ClearedFresh),
            "cleared_stale" => Some(Self::ClearedStale),
            "unknown_unavailable" => Some(Self::UnknownUnavailable),
            _ => None,
        }
    }

    fn id(self) -> &'static str {
        match self {
            Self::ActiveFresh => "active_fresh",
            Self::ActiveStale => "active_stale",
            Self::ClearedFresh => "cleared_fresh",
            Self::ClearedStale => "cleared_stale",
            Self::UnknownUnavailable => "unknown_unavailable",
        }
    }
}

fn demo_territory(code: &str) -> Option<DemoTerritory> {
    match code {
        "UA-46" => Some(DemoTerritory {
            code: "UA-46",
            name_uk: "Львівська область",
        }),
        "UA-30" => Some(DemoTerritory {
            code: "UA-30",
            name_uk: "м. Київ",
        }),
        _ => None,
    }
}

fn state_name(state: AlertState) -> &'static str {
    match state {
        AlertState::Unknown => "unknown",
        AlertState::Active => "active",
        AlertState::Cleared => "cleared",
    }
}

fn freshness_name(freshness: SourceFreshness) -> &'static str {
    match freshness {
        SourceFreshness::Unknown => "unknown",
        SourceFreshness::Fresh => "fresh",
        SourceFreshness::Stale => "stale",
        SourceFreshness::Unavailable => "unavailable",
    }
}

fn presentation_name(state: PresentationState) -> &'static str {
    match state {
        PresentationState::ActiveCurrent => "active_current",
        PresentationState::ActiveLastKnown => "active_last_known",
        PresentationState::ClearNeedsRefresh => "clear_needs_refresh",
        PresentationState::Unknown => "unknown",
        PresentationState::ClearedCurrent => "cleared_current",
    }
}

fn apply_demo_event(
    status: &mut RegionStatus,
    territory: DemoTerritory,
    scenario: DemoScenario,
    issued_at: i64,
    signal: Signal,
) {
    let phase = match signal {
        Signal::Start => "start",
        Signal::Clear => "clear",
    };
    let event = OfficialEvent {
        region: territory.code.into(),
        event_id: format!(
            "synthetic:ui:{}:{}:{}:{}",
            territory.code,
            scenario.id(),
            issued_at,
            phase
        ),
        issued_at,
        signal,
        verified_official: true,
    };

    let updated = status.apply(&event);
    debug_assert!(updated);
}

fn demo_snapshot_json(territory_code: &str, scenario_id: &str) -> Result<String, &'static str> {
    let territory = demo_territory(territory_code).ok_or("unknown synthetic territory")?;
    let scenario = DemoScenario::parse(scenario_id).ok_or("unknown synthetic scenario")?;
    let mut status = RegionStatus::default();

    match scenario {
        DemoScenario::ActiveFresh => {
            apply_demo_event(&mut status, territory, scenario, 100, Signal::Start);
            status.set_source_freshness(SourceFreshness::Fresh);
        }
        DemoScenario::ActiveStale => {
            apply_demo_event(&mut status, territory, scenario, 100, Signal::Start);
            status.set_source_freshness(SourceFreshness::Stale);
        }
        DemoScenario::ClearedFresh => {
            apply_demo_event(&mut status, territory, scenario, 100, Signal::Start);
            apply_demo_event(&mut status, territory, scenario, 200, Signal::Clear);
            status.set_source_freshness(SourceFreshness::Fresh);
        }
        DemoScenario::ClearedStale => {
            apply_demo_event(&mut status, territory, scenario, 100, Signal::Start);
            apply_demo_event(&mut status, territory, scenario, 200, Signal::Clear);
            status.set_source_freshness(SourceFreshness::Stale);
        }
        DemoScenario::UnknownUnavailable => {
            status.set_source_freshness(SourceFreshness::Unavailable);
        }
    }

    let presentation = presentation_state(&status);

    Ok(format!(
        concat!(
            "{{",
            "\"region\":\"{}\",",
            "\"territory_name\":\"{}\",",
            "\"scenario\":\"{}\",",
            "\"confirmed_state\":\"{}\",",
            "\"source_freshness\":\"{}\",",
            "\"presentation\":\"{}\",",
            "\"issued_at\":{},",
            "\"app_version\":\"{}\",",
            "\"synthetic\":true",
            "}}"
        ),
        territory.code,
        territory.name_uk,
        scenario.id(),
        state_name(status.state),
        freshness_name(status.source_freshness),
        presentation_name(presentation),
        status.last_event_at.unwrap_or_default(),
        env!("CARGO_PKG_VERSION"),
    ))
}

#[tauri::command]
fn demo_snapshot(territory: String, scenario: String) -> Result<String, String> {
    demo_snapshot_json(&territory, &scenario).map_err(str::to_owned)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![demo_snapshot])
        .run(tauri::generate_context!())
        .expect("failed to run ТривогА shell");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(territory: &str, scenario: &str) -> String {
        demo_snapshot_json(territory, scenario).expect("known synthetic fixture")
    }

    #[test]
    fn active_stale_preserves_last_known_alert() {
        let payload = payload("UA-46", "active_stale");

        assert!(payload.contains("\"region\":\"UA-46\""));
        assert!(payload.contains("\"territory_name\":\"Львівська область\""));
        assert!(payload.contains("\"confirmed_state\":\"active\""));
        assert!(payload.contains("\"source_freshness\":\"stale\""));
        assert!(payload.contains("\"presentation\":\"active_last_known\""));
        assert!(payload.contains("\"synthetic\":true"));
        assert!(payload.contains("\"app_version\":\"0.1.0\""));
        assert!(!payload.contains("\"presentation\":\"cleared_current\""));
    }

    #[test]
    fn active_fresh_is_current() {
        let payload = payload("UA-30", "active_fresh");

        assert!(payload.contains("\"region\":\"UA-30\""));
        assert!(payload.contains("\"territory_name\":\"м. Київ\""));
        assert!(payload.contains("\"confirmed_state\":\"active\""));
        assert!(payload.contains("\"source_freshness\":\"fresh\""));
        assert!(payload.contains("\"presentation\":\"active_current\""));
    }

    #[test]
    fn fresh_clear_is_current_clear() {
        let payload = payload("UA-46", "cleared_fresh");

        assert!(payload.contains("\"confirmed_state\":\"cleared\""));
        assert!(payload.contains("\"source_freshness\":\"fresh\""));
        assert!(payload.contains("\"presentation\":\"cleared_current\""));
        assert!(payload.contains("\"issued_at\":200"));
    }

    #[test]
    fn stale_clear_requires_refresh_and_is_not_current_clear() {
        let payload = payload("UA-46", "cleared_stale");

        assert!(payload.contains("\"confirmed_state\":\"cleared\""));
        assert!(payload.contains("\"source_freshness\":\"stale\""));
        assert!(payload.contains("\"presentation\":\"clear_needs_refresh\""));
        assert!(!payload.contains("\"presentation\":\"cleared_current\""));
    }

    #[test]
    fn unavailable_without_event_stays_unknown() {
        let payload = payload("UA-30", "unknown_unavailable");

        assert!(payload.contains("\"confirmed_state\":\"unknown\""));
        assert!(payload.contains("\"source_freshness\":\"unavailable\""));
        assert!(payload.contains("\"presentation\":\"unknown\""));
        assert!(payload.contains("\"issued_at\":0"));
    }

    #[test]
    fn unknown_territory_is_rejected() {
        assert_eq!(
            demo_snapshot_json("UA-99", "active_stale"),
            Err("unknown synthetic territory")
        );
    }

    #[test]
    fn unknown_scenario_is_rejected() {
        assert_eq!(
            demo_snapshot_json("UA-46", "all_clear"),
            Err("unknown synthetic scenario")
        );
    }
}
