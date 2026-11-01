//! ТривогА: offline deterministic model. No network, notifications or production integration.
//! All timestamps are integer UTC epoch seconds supplied by caller.

pub mod presentation;
pub mod store;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertState {
    Unknown,
    Active,
    Cleared,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signal {
    Start,
    Clear,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OfficialEvent {
    pub region: String,
    /// Unique normalized event-version identity. Do not pass a mutable provider record id directly.
    pub event_id: String,
    pub issued_at: i64,
    pub signal: Signal,
    /// Only trusted official adapters may set this flag, after source admission.
    pub verified_official: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceFreshness {
    Unknown,
    Fresh,
    Stale,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionStatus {
    pub state: AlertState,
    pub last_event_at: Option<i64>,
    pub last_event_id: Option<String>,
    pub source_freshness: SourceFreshness,
}

impl Default for RegionStatus {
    fn default() -> Self {
        Self {
            state: AlertState::Unknown,
            last_event_at: None,
            last_event_id: None,
            source_freshness: SourceFreshness::Unknown,
        }
    }
}

impl RegionStatus {
    /// Compatibility helper for early callers.
    pub fn set_source_health(&mut self, healthy: bool) {
        self.source_freshness = if healthy {
            SourceFreshness::Fresh
        } else {
            SourceFreshness::Unavailable
        };
    }

    pub fn set_source_freshness(&mut self, freshness: SourceFreshness) {
        self.source_freshness = freshness;
    }

    /// Returns true only when the official state was updated.
    /// Equal-time conflicting signals are rejected conservatively.
    pub fn apply(&mut self, event: &OfficialEvent) -> bool {
        if !event.verified_official || event.region.trim().is_empty() || event.event_id.trim().is_empty() {
            return false;
        }
        if let Some(prev) = self.last_event_at {
            if event.issued_at <= prev { return false; }
        }
        self.state = match event.signal { Signal::Start => AlertState::Active, Signal::Clear => AlertState::Cleared };
        self.last_event_at = Some(event.issued_at);
        self.last_event_id = Some(event.event_id.clone());
        true
    }

    /// Only Fresh means the source can currently confirm new information.
    pub fn current_data_available(&self) -> bool {
        self.source_freshness == SourceFreshness::Fresh
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Advisory {
    pub event_id: String,
    pub expires_at: i64,
}

impl Advisory {
    /// Expiry only removes a supplementary advisory. It never clears official alert state.
    pub fn is_current(&self, now: i64) -> bool { now < self.expires_at }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ev(id: &str, at: i64, signal: Signal, official: bool) -> OfficialEvent {
        OfficialEvent { region: "UA-46".into(), event_id: id.into(), issued_at: at, signal, verified_official: official }
    }
    #[test] fn starts_and_clears_only_with_newer_verified_event() {
        let mut s = RegionStatus::default();
        assert!(s.apply(&ev("start", 10, Signal::Start, true)));
        assert_eq!(s.state, AlertState::Active);
        assert!(!s.apply(&ev("unverified-clear", 11, Signal::Clear, false)));
        assert!(!s.apply(&ev("older-clear", 9, Signal::Clear, true)));
        assert_eq!(s.state, AlertState::Active);
        assert!(s.apply(&ev("clear", 12, Signal::Clear, true)));
        assert_eq!(s.state, AlertState::Cleared);
    }
    #[test] fn duplicate_and_equal_timestamp_conflict_are_ignored() {
        let mut s = RegionStatus::default();
        assert!(s.apply(&ev("a", 10, Signal::Start, true)));
        assert!(!s.apply(&ev("a", 10, Signal::Start, true)));
        assert!(!s.apply(&ev("b", 10, Signal::Clear, true)));
        assert_eq!(s.state, AlertState::Active);
    }
    #[test] fn feed_loss_never_clears_alert() {
        let mut s = RegionStatus::default();
        s.set_source_health(true);
        assert!(s.apply(&ev("a", 10, Signal::Start, true)));
        s.set_source_health(false);
        assert_eq!(s.state, AlertState::Active);
        assert_eq!(s.source_freshness, SourceFreshness::Unavailable);
        assert!(!s.current_data_available());
    }

    #[test] fn unknown_stale_and_unavailable_are_distinct() {
        let mut s = RegionStatus::default();
        assert_eq!(s.source_freshness, SourceFreshness::Unknown);

        s.set_source_freshness(SourceFreshness::Stale);
        assert_eq!(s.source_freshness, SourceFreshness::Stale);
        assert!(!s.current_data_available());

        s.set_source_freshness(SourceFreshness::Unavailable);
        assert_eq!(s.source_freshness, SourceFreshness::Unavailable);
        assert!(!s.current_data_available());

        s.set_source_freshness(SourceFreshness::Fresh);
        assert!(s.current_data_available());
    }
    #[test] fn advisory_expiry_is_independent() {
        let mut s = RegionStatus::default();
        assert!(s.apply(&ev("a", 10, Signal::Start, true)));
        let a = Advisory { event_id: "advisory".into(), expires_at: 30 };
        assert!(a.is_current(29));
        assert!(!a.is_current(30));
        assert_eq!(s.state, AlertState::Active);
    }
    #[test] fn invalid_and_blank_events_rejected() {
        let mut s = RegionStatus::default();
        let mut e = ev("", 10, Signal::Clear, true);
        assert!(!s.apply(&e));
        e.event_id = "a".into(); e.region = " ".into();
        assert!(!s.apply(&e));
        assert_eq!(s.state, AlertState::Unknown);
    }
}
