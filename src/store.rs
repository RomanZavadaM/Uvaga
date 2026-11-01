use std::collections::{BTreeMap, VecDeque};

use crate::{AlertState, OfficialEvent, RegionStatus, SourceFreshness};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotRegion {
    pub region: String,
    pub state: AlertState,
    pub last_event_at: Option<i64>,
    pub last_event_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoverySnapshot {
    pub version: u8,
    pub regions: Vec<SnapshotRegion>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotError {
    UnsupportedVersion,
    BlankRegion,
    DuplicateRegion,
    InconsistentLastEvent,
    StateEventMismatch,
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

    /// Export only the minimum confirmed state needed to resume conservatively.
    /// Source freshness is intentionally NOT persisted: after restart it must be Unknown
    /// until a source is contacted again.
    pub fn recovery_snapshot(&self) -> RecoverySnapshot {
        RecoverySnapshot {
            version: 1,
            regions: self.regions.iter().map(|(region, record)| SnapshotRegion {
                region: region.clone(),
                state: record.status.state,
                last_event_at: record.status.last_event_at,
                last_event_id: record.status.last_event_id.clone(),
            }).collect(),
        }
    }

    pub fn from_recovery_snapshot(snapshot: &RecoverySnapshot) -> Result<Self, SnapshotError> {
        if snapshot.version != 1 {
            return Err(SnapshotError::UnsupportedVersion);
        }

        let mut regions = BTreeMap::new();
        for item in &snapshot.regions {
            if item.region.trim().is_empty() {
                return Err(SnapshotError::BlankRegion);
            }
            if item.last_event_at.is_some() != item.last_event_id.is_some() {
                return Err(SnapshotError::InconsistentLastEvent);
            }
            let has_event = item.last_event_at.is_some();
            if (item.state == AlertState::Unknown && has_event)
                || (item.state != AlertState::Unknown && !has_event)
            {
                return Err(SnapshotError::StateEventMismatch);
            }
            if regions.contains_key(&item.region) {
                return Err(SnapshotError::DuplicateRegion);
            }

            let mut recent_event_ids = VecDeque::new();
            if let Some(id) = &item.last_event_id {
                if id.trim().is_empty() {
                    return Err(SnapshotError::InconsistentLastEvent);
                }
                recent_event_ids.push_back(id.clone());
            }

            regions.insert(item.region.clone(), RegionRecord {
                status: RegionStatus {
                    state: item.state,
                    last_event_at: item.last_event_at,
                    last_event_id: item.last_event_id.clone(),
                    source_freshness: SourceFreshness::Unknown,
                },
                recent_event_ids,
            });
        }

        Ok(Self { regions })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AlertState, OfficialEvent, Signal, SourceFreshness};

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
    fn recovery_preserves_confirmed_state_but_resets_freshness() {
        let mut book = AlertBook::default();
        assert_eq!(
            book.apply(&ev("UA-46", "start-1", 10, Signal::Start)),
            ApplyResult::Applied
        );
        book.set_source_health("UA-46", true);

        let snapshot = book.recovery_snapshot();
        let restored = AlertBook::from_recovery_snapshot(&snapshot).unwrap();
        let status = restored.status("UA-46").unwrap();

        assert_eq!(status.state, AlertState::Active);
        assert_eq!(status.last_event_at, Some(10));
        assert_eq!(status.last_event_id.as_deref(), Some("start-1"));
        assert_eq!(status.source_freshness, SourceFreshness::Unknown);
    }

    #[test]
    fn recovery_rejects_inconsistent_or_duplicate_records() {
        let inconsistent = RecoverySnapshot {
            version: 1,
            regions: vec![SnapshotRegion {
                region: "UA-46".into(),
                state: AlertState::Active,
                last_event_at: Some(10),
                last_event_id: None,
            }],
        };
        assert_eq!(
            AlertBook::from_recovery_snapshot(&inconsistent).unwrap_err(),
            SnapshotError::InconsistentLastEvent
        );

        let duplicate = RecoverySnapshot {
            version: 1,
            regions: vec![
                SnapshotRegion {
                    region: "UA-46".into(),
                    state: AlertState::Active,
                    last_event_at: Some(10),
                    last_event_id: Some("a".into()),
                },
                SnapshotRegion {
                    region: "UA-46".into(),
                    state: AlertState::Cleared,
                    last_event_at: Some(11),
                    last_event_id: Some("b".into()),
                },
            ],
        };
        assert_eq!(
            AlertBook::from_recovery_snapshot(&duplicate).unwrap_err(),
            SnapshotError::DuplicateRegion
        );
    }

    #[test]
    fn restored_last_event_is_still_deduplicated() {
        let snapshot = RecoverySnapshot {
            version: 1,
            regions: vec![SnapshotRegion {
                region: "UA-46".into(),
                state: AlertState::Active,
                last_event_at: Some(10),
                last_event_id: Some("same-id".into()),
            }],
        };
        let mut restored = AlertBook::from_recovery_snapshot(&snapshot).unwrap();

        assert_eq!(
            restored.apply(&ev("UA-46", "same-id", 20, Signal::Clear)),
            ApplyResult::Duplicate
        );
        assert_eq!(restored.status("UA-46").unwrap().state, AlertState::Active);
    }

    #[test]
    fn recovery_rejects_state_event_mismatch() {
        let active_without_event = RecoverySnapshot {
            version: 1,
            regions: vec![SnapshotRegion {
                region: "UA-46".into(),
                state: AlertState::Active,
                last_event_at: None,
                last_event_id: None,
            }],
        };
        assert_eq!(
            AlertBook::from_recovery_snapshot(&active_without_event).unwrap_err(),
            SnapshotError::StateEventMismatch
        );

        let unknown_with_event = RecoverySnapshot {
            version: 1,
            regions: vec![SnapshotRegion {
                region: "UA-46".into(),
                state: AlertState::Unknown,
                last_event_at: Some(10),
                last_event_id: Some("a".into()),
            }],
        };
        assert_eq!(
            AlertBook::from_recovery_snapshot(&unknown_with_event).unwrap_err(),
            SnapshotError::StateEventMismatch
        );
    }

    #[test]
    fn unsupported_snapshot_version_fails_closed() {
        let snapshot = RecoverySnapshot {
            version: 2,
            regions: Vec::new(),
        };
        assert_eq!(
            AlertBook::from_recovery_snapshot(&snapshot).unwrap_err(),
            SnapshotError::UnsupportedVersion
        );
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
