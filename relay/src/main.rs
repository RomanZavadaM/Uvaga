use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use uvaga_core::{
    presentation::{presentation_state, PresentationState},
    store::AlertBook,
    AlertState, OfficialEvent, Signal, SourceFreshness,
};

#[derive(Clone)]
struct AppState {
    book: Arc<RwLock<AlertBook>>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    mode: &'static str,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct RegionSnapshot {
    region: String,
    confirmed_state: &'static str,
    source_freshness: &'static str,
    presentation: &'static str,
    synthetic: bool,
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

fn demo_book() -> AlertBook {
    let mut book = AlertBook::default();

    let lviv = OfficialEvent {
        region: "UA-46".into(),
        event_id: "synthetic:lviv-start".into(),
        issued_at: 100,
        signal: Signal::Start,
        verified_official: true,
    };
    let kyiv = OfficialEvent {
        region: "UA-80".into(),
        event_id: "synthetic:kyiv-clear".into(),
        issued_at: 100,
        signal: Signal::Clear,
        verified_official: true,
    };

    let _ = book.apply(&lviv);
    let _ = book.apply(&kyiv);
    book.set_source_freshness("UA-46", SourceFreshness::Stale);
    book.set_source_freshness("UA-80", SourceFreshness::Fresh);
    book
}

fn snapshot_for(book: &AlertBook, region: &str) -> Option<RegionSnapshot> {
    let status = book.status(region)?;
    Some(RegionSnapshot {
        region: region.to_owned(),
        confirmed_state: state_name(status.state),
        source_freshness: freshness_name(status.source_freshness),
        presentation: presentation_name(presentation_state(status)),
        synthetic: true,
    })
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        mode: "synthetic-local-only",
    })
}

async fn region_status(
    Path(region): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<RegionSnapshot>, StatusCode> {
    let book = state
        .book
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    snapshot_for(&book, &region)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        book: Arc::new(RwLock::new(demo_book())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/demo/status/{region}", get(region_status))
        .with_state(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 8787));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind local synthetic relay");

    println!("ТривогА synthetic relay: http://{address}");
    axum::serve(listener, app)
        .await
        .expect("local synthetic relay failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_snapshot_preserves_active_but_stale_state() {
        let book = demo_book();
        let snapshot = snapshot_for(&book, "UA-46").unwrap();

        assert_eq!(snapshot.confirmed_state, "active");
        assert_eq!(snapshot.source_freshness, "stale");
        assert_eq!(snapshot.presentation, "active_last_known");
        assert!(snapshot.synthetic);
    }

    #[test]
    fn synthetic_snapshot_can_show_fresh_confirmed_clear() {
        let book = demo_book();
        let snapshot = snapshot_for(&book, "UA-80").unwrap();

        assert_eq!(snapshot.confirmed_state, "cleared");
        assert_eq!(snapshot.source_freshness, "fresh");
        assert_eq!(snapshot.presentation, "cleared_current");
    }

    #[test]
    fn stale_clear_uses_core_clear_needs_refresh_presentation() {
        let mut book = demo_book();
        book.set_source_freshness("UA-80", SourceFreshness::Stale);

        let snapshot = snapshot_for(&book, "UA-80").unwrap();
        assert_eq!(snapshot.confirmed_state, "cleared");
        assert_eq!(snapshot.source_freshness, "stale");
        assert_eq!(snapshot.presentation, "clear_needs_refresh");
    }

    #[test]
    fn unknown_region_is_not_silently_clear() {
        let book = demo_book();
        assert_eq!(snapshot_for(&book, "UA-XX"), None);
    }
}
