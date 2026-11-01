use uvaga_core::{
    presentation::{presentation_state, PresentationState},
    AlertState, OfficialEvent, RegionStatus, Signal, SourceFreshness,
};

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

fn demo_snapshot_json() -> String {
    let mut status = RegionStatus::default();
    status.set_source_freshness(SourceFreshness::Stale);

    let event = OfficialEvent {
        region: "UA-46".into(),
        event_id: "synthetic:ui-bridge:100:start".into(),
        issued_at: 100,
        signal: Signal::Start,
        verified_official: true,
    };

    let updated = status.apply(&event);
    debug_assert!(updated);

    let presentation = presentation_state(&status);

    format!(
        concat!(
            "{{",
            "\"region\":\"UA-46\",",
            "\"confirmed_state\":\"{}\",",
            "\"source_freshness\":\"{}\",",
            "\"presentation\":\"{}\",",
            "\"issued_at\":{},",
            "\"synthetic\":true",
            "}}"
        ),
        state_name(status.state),
        freshness_name(status.source_freshness),
        presentation_name(presentation),
        status.last_event_at.unwrap_or_default(),
    )
}

#[tauri::command]
fn demo_snapshot() -> String {
    demo_snapshot_json()
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

    #[test]
    fn demo_snapshot_uses_fail_safe_core_state() {
        let payload = demo_snapshot_json();

        assert!(payload.contains("\"region\":\"UA-46\""));
        assert!(payload.contains("\"confirmed_state\":\"active\""));
        assert!(payload.contains("\"source_freshness\":\"stale\""));
        assert!(payload.contains("\"presentation\":\"active_last_known\""));
        assert!(payload.contains("\"synthetic\":true"));
        assert!(!payload.contains("\"presentation\":\"cleared_current\""));
    }
}
