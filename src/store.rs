use std::collections::{BTreeMap, VecDeque};

use crate::{OfficialEvent, RegionStatus};

const MAX_RECENT_EVENT_IDS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplyResult {
    Applied,
    Duplicate,
    Rejected,
}

#[derive(Clone, Debug)]
struct RegionRecord {
    status: RegionStatus,
    recent_event_ids: VecDeque<String>,
}

impl Default for RegionRecord {
    fn default() -> Self {
        Self {
            status: RegionStatus::default(),
            recent_event_ids: VecDeque::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AlertBook {
    regions: BTreeMap<String, RegionRecord>,
}

impl AlertBook {
    pub fn apply(&mut self, event: &OfficialEvent) -> ApplyResult {
        if event.region.trim().is_empty() || event.event_id.trim().is_empty() {
            return ApplyResult::Rejected;
        }

        let record = self.regions.entry(event.region.clone()).or_default();
        if record.recent_event_ids.iter().any(|id| id == &event.event_id) {
            return ApplyResult::Duplicate;
        }

        if !record.status.apply(event) {
            return ApplyResult::Rejected;
        }

        record.recent_event_ids.push_back(event.event_id.clone());
        if record.recent_event_ids.len() > MAX_RECENT_EVENT_IDS {
            record.recent_event_ids.pop_front();
        }

        ApplyResult::Applied
    }

    pub fn status(&self, region: &str) -> Option<&RegionStatus> {
        self.regions.get(region).map(|record| &record.status)
    }

    pub fn set_source_health(&mut self, region: &str, healthy: bool) {
        self.regions
            .entry(region.to_owned())
            .or_default()
            .status
            .set_source_health(healthy);
    }

    pub fn region_count(&self) -> usize {
        self.regions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AlertState, OfficialEvent, Signal};

    fn ev(region: &str, id: &str, at: i64, signal: Signal) -> OfficialEvent {
        OfficialEvent {
            region: region.into(),
            event_id: id.into(),
            issued_at: at,
            signal,
            verified_official: true,
        }
    }

    #[test]
    fn keeps_regions_independent() {
        let mut book = AlertBook::default();

        assert_eq!(
            book.apply(&ev("UA-46", "lviv-start", 10, Signal::Start)),
            ApplyResult::Applied
        );
        assert_eq!(
            book.apply(&ev("UA-80", "kyiv-start", 11, Signal::Start)),
            ApplyResult::Applied
        );
        assert_eq!(
            book.apply(&ev("UA-46", "lviv-clear", 12, Signal::Clear)),
            ApplyResult::Applied
        );

        assert_eq!(book.status("UA-46").unwrap().state, AlertState::Cleared);
        assert_eq!(book.status("UA-80").unwrap().state, AlertState::Active);
        assert_eq!(book.region_count(), 2);
    }

    #[test]
    fn rejects_replayed_event_id_even_with_newer_timestamp() {
        let mut book = AlertBook::default();

        assert_eq!(
            book.apply(&ev("UA-46", "same-id", 10, Signal::Start)),
            ApplyResult::Applied
        );
        assert_eq!(
            book.apply(&ev("UA-46", "same-id", 20, Signal::Clear)),
            ApplyResult::Duplicate
        );
        assert_eq!(book.status("UA-46").unwrap().state, AlertState::Active);
    }

    #[test]
    fn same_event_id_in_different_regions_is_not_cross_deduplicated() {
        let mut book = AlertBook::default();

        assert_eq!(
            book.apply(&ev("UA-46", "provider-1", 10, Signal::Start)),
            ApplyResult::Applied
        );
        assert_eq!(
            book.apply(&ev("UA-80", "provider-1", 10, Signal::Start)),
            ApplyResult::Applied
        );
    }

    #[test]
    fn source_health_does_not_change_confirmed_state() {
        let mut book = AlertBook::default();

        assert_eq!(
            book.apply(&ev("UA-46", "a", 10, Signal::Start)),
            ApplyResult::Applied
        );
        book.set_source_health("UA-46", false);

        let status = book.status("UA-46").unwrap();
        assert_eq!(status.state, AlertState::Active);
        assert!(!status.current_data_available());
    }

    #[test]
    fn unverified_event_is_rejected_and_not_poisoned_as_duplicate() {
        let mut book = AlertBook::default();
        let mut event = ev("UA-46", "a", 10, Signal::Start);
        event.verified_official = false;

        assert_eq!(book.apply(&event), ApplyResult::Rejected);

        event.verified_official = true;
        assert_eq!(book.apply(&event), ApplyResult::Applied);
    }
}
