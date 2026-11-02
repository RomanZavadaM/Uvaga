//! Offline-only, independently scoped synthetic official threat states.
//! No provider adapters, networking, credentials, live geolocation or notifications.
use std::collections::BTreeMap;
use crate::{AlertState, OfficialEvent, RegionStatus, Signal, SourceFreshness};
use crate::presentation::{presentation_state, PresentationState};
use crate::territory::{TerritoryCatalog, TerritoryKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatCategory { AirRaid, Artillery, Chemical, Radiation, CivilOther, Unspecified }

#[derive(Clone, Debug)]
pub struct TerritoryScopedEvent {
    pub territory: TerritoryKey,
    pub category: ThreatCategory,
    pub event_id: String,
    pub issued_at: i64,
    pub signal: Signal,
    pub verified_official: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopedResult { Applied, Rejected }

#[derive(Clone, Debug, Default)]
pub struct ScopedThreatBook {
    statuses: BTreeMap<(TerritoryKey, ThreatCategory), RegionStatus>,
}

impl ScopedThreatBook {
    /// No implicit parent/descendant updates: an oblast clear cannot clear a hromada alert.
    /// Events are accepted only for catalogued, explicitly scoped territories.
    pub fn apply(&mut self, catalog: &TerritoryCatalog, event: &TerritoryScopedEvent) -> ScopedResult {
        if catalog.get(&event.territory).is_none()
            || event.event_id.trim().is_empty()
            || !event.verified_official
        {
            return ScopedResult::Rejected;
        }
        let key = (event.territory.clone(), event.category);
        let mut candidate = self.statuses.get(&key).cloned().unwrap_or_default();
        let official = OfficialEvent {
            region: event.territory.code.clone(),
            event_id: event.event_id.clone(),
            issued_at: event.issued_at,
            signal: event.signal,
            verified_official: event.verified_official,
        };
        if !candidate.apply(&official) { return ScopedResult::Rejected; }
        self.statuses.insert(key, candidate);
        ScopedResult::Applied
    }

    pub fn set_freshness(&mut self, territory: &TerritoryKey, category: ThreatCategory, freshness: SourceFreshness) {
        if let Some(status) = self.statuses.get_mut(&(territory.clone(), category)) {
            status.set_source_freshness(freshness);
        }
    }

    pub fn status(&self, territory: &TerritoryKey, category: ThreatCategory) -> Option<&RegionStatus> {
        self.statuses.get(&(territory.clone(), category))
    }

    pub fn presentation(&self, territory: &TerritoryKey, category: ThreatCategory) -> PresentationState {
        self.status(territory, category).map(presentation_state).unwrap_or(PresentationState::Unknown)
    }

    pub fn has_active_last_known(&self, territory: &TerritoryKey) -> bool {
        self.statuses.iter().any(|((key, _), status)| key == territory && status.state == AlertState::Active)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::territory::{TerritoryLevel, TerritoryNode};
    fn key(level: TerritoryLevel, code: &str) -> TerritoryKey {
        TerritoryKey { level, code: code.into() }
    }
    fn catalog() -> (TerritoryCatalog, TerritoryKey, TerritoryKey, TerritoryKey) {
        let mut c = TerritoryCatalog::default();
        let ua = key(TerritoryLevel::Country, "UA");
        let oblast = key(TerritoryLevel::Region, "UA-46");
        let raion = key(TerritoryLevel::Raion, "TEST-RAION");
        let hromada = key(TerritoryLevel::Hromada, "TEST-HROMADA");
        for (k, parent) in [
            (ua.clone(), None),
            (oblast.clone(), Some(ua.clone())),
            (raion.clone(), Some(oblast.clone())),
            (hromada.clone(), Some(raion)),
        ] {
            c.insert(TerritoryNode { name_uk: "Синтетична територія".into(), key: k, parent }).unwrap();
        }
        (c, ua, oblast, hromada)
    }
    fn event(territory: &TerritoryKey, category: ThreatCategory, id: &str, at: i64, signal: Signal) -> TerritoryScopedEvent {
        TerritoryScopedEvent { territory: territory.clone(), category, event_id: id.into(), issued_at: at, signal, verified_official: true }
    }

    #[test]
    fn simultaneous_categories_cannot_clear_one_another() {
        let (catalog, _, oblast, _) = catalog();
        let mut book = ScopedThreatBook::default();
        assert_eq!(book.apply(&catalog, &event(&oblast, ThreatCategory::AirRaid, "air-start", 10, Signal::Start)), ScopedResult::Applied);
        assert_eq!(book.apply(&catalog, &event(&oblast, ThreatCategory::Chemical, "chem-start", 11, Signal::Start)), ScopedResult::Applied);
        assert_eq!(book.apply(&catalog, &event(&oblast, ThreatCategory::Chemical, "chem-clear", 12, Signal::Clear)), ScopedResult::Applied);
        assert_eq!(book.status(&oblast, ThreatCategory::AirRaid).unwrap().state, AlertState::Active);
        assert_eq!(book.status(&oblast, ThreatCategory::Chemical).unwrap().state, AlertState::Cleared);
        assert!(book.has_active_last_known(&oblast));
    }

    #[test]
    fn oblast_clear_never_erases_hromada_alert() {
        let (catalog, _, oblast, hromada) = catalog();
        let mut book = ScopedThreatBook::default();
        assert_eq!(book.apply(&catalog, &event(&hromada, ThreatCategory::AirRaid, "h-start", 10, Signal::Start)), ScopedResult::Applied);
        assert_eq!(book.apply(&catalog, &event(&oblast, ThreatCategory::AirRaid, "o-start", 11, Signal::Start)), ScopedResult::Applied);
        assert_eq!(book.apply(&catalog, &event(&oblast, ThreatCategory::AirRaid, "o-clear", 12, Signal::Clear)), ScopedResult::Applied);
        assert_eq!(book.status(&hromada, ThreatCategory::AirRaid).unwrap().state, AlertState::Active);
        assert!(book.has_active_last_known(&hromada));
    }

    #[test]
    fn stale_clear_is_never_safe() {
        let (catalog, _, oblast, _) = catalog();
        let mut book = ScopedThreatBook::default();
        book.apply(&catalog, &event(&oblast, ThreatCategory::AirRaid, "start", 10, Signal::Start));
        book.apply(&catalog, &event(&oblast, ThreatCategory::AirRaid, "clear", 11, Signal::Clear));
        book.set_freshness(&oblast, ThreatCategory::AirRaid, SourceFreshness::Stale);
        assert_eq!(book.presentation(&oblast, ThreatCategory::AirRaid), PresentationState::ClearNeedsRefresh);
        assert!(!book.presentation(&oblast, ThreatCategory::AirRaid).may_show_as_safe());
    }

    #[test]
    fn rejects_unmapped_unverified_replayed_and_equal_time_events() {
        let (catalog, _, oblast, _) = catalog();
        let mut book = ScopedThreatBook::default();
        let unknown = key(TerritoryLevel::Hromada, "UNKNOWN");
        assert_eq!(book.apply(&catalog, &event(&unknown, ThreatCategory::AirRaid, "unknown", 1, Signal::Start)), ScopedResult::Rejected);
        let mut unverified = event(&oblast, ThreatCategory::AirRaid, "bad", 2, Signal::Clear);
        unverified.verified_official = false;
        assert_eq!(book.apply(&catalog, &unverified), ScopedResult::Rejected);
        let start = event(&oblast, ThreatCategory::AirRaid, "start", 3, Signal::Start);
        assert_eq!(book.apply(&catalog, &start), ScopedResult::Applied);
        assert_eq!(book.apply(&catalog, &start), ScopedResult::Rejected);
        assert_eq!(book.apply(&catalog, &event(&oblast, ThreatCategory::AirRaid, "equal-clear", 3, Signal::Clear)), ScopedResult::Rejected);
        assert_eq!(book.apply(&catalog, &event(&oblast, ThreatCategory::AirRaid, "old-clear", 2, Signal::Clear)), ScopedResult::Rejected);
        assert_eq!(book.status(&oblast, ThreatCategory::AirRaid).unwrap().state, AlertState::Active);
    }
}
