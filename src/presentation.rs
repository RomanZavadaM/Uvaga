use crate::{AlertState, RegionStatus, SourceFreshness};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresentationState {
    /// Confirmed active alert and source is currently fresh.
    ActiveCurrent,
    /// Last confirmed state is active, but source freshness is not current.
    ActiveLastKnown,
    /// Last confirmed state was clear, but it cannot be presented as currently safe.
    ClearNeedsRefresh,
    /// No confirmed official state exists yet.
    Unknown,
    /// Confirmed clear from a currently fresh source.
    ClearedCurrent,
}

impl PresentationState {
    /// Larger value means the item belongs higher in an attention-oriented list.
    /// This is presentation priority only, not a prediction of threat severity.
    pub fn attention_rank(self) -> u8 {
        match self {
            PresentationState::ActiveLastKnown => 5,
            PresentationState::ActiveCurrent => 4,
            PresentationState::ClearNeedsRefresh => 3,
            PresentationState::Unknown => 2,
            PresentationState::ClearedCurrent => 1,
        }
    }

    pub fn may_show_as_safe(self) -> bool {
        self == PresentationState::ClearedCurrent
    }
}

pub fn presentation_state(status: &RegionStatus) -> PresentationState {
    match status.state {
        AlertState::Active => {
            if status.source_freshness == SourceFreshness::Fresh {
                PresentationState::ActiveCurrent
            } else {
                PresentationState::ActiveLastKnown
            }
        }
        AlertState::Cleared => {
            if status.source_freshness == SourceFreshness::Fresh {
                PresentationState::ClearedCurrent
            } else {
                PresentationState::ClearNeedsRefresh
            }
        }
        AlertState::Unknown => PresentationState::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RegionStatus;

    fn status(state: AlertState, freshness: SourceFreshness) -> RegionStatus {
        RegionStatus {
            state,
            last_event_at: None,
            last_event_id: None,
            source_freshness: freshness,
        }
    }

    #[test]
    fn active_never_becomes_safe_when_source_is_lost() {
        for freshness in [
            SourceFreshness::Unknown,
            SourceFreshness::Stale,
            SourceFreshness::Unavailable,
        ] {
            let p = presentation_state(&status(AlertState::Active, freshness));
            assert_eq!(p, PresentationState::ActiveLastKnown);
            assert!(!p.may_show_as_safe());
        }
    }

    #[test]
    fn stale_clear_is_not_safe() {
        for freshness in [
            SourceFreshness::Unknown,
            SourceFreshness::Stale,
            SourceFreshness::Unavailable,
        ] {
            let p = presentation_state(&status(AlertState::Cleared, freshness));
            assert_eq!(p, PresentationState::ClearNeedsRefresh);
            assert!(!p.may_show_as_safe());
        }
    }

    #[test]
    fn only_fresh_confirmed_clear_may_show_as_safe() {
        let p = presentation_state(&status(AlertState::Cleared, SourceFreshness::Fresh));
        assert_eq!(p, PresentationState::ClearedCurrent);
        assert!(p.may_show_as_safe());
    }

    #[test]
    fn unknown_state_is_never_safe_even_with_fresh_source() {
        let p = presentation_state(&status(AlertState::Unknown, SourceFreshness::Fresh));
        assert_eq!(p, PresentationState::Unknown);
        assert!(!p.may_show_as_safe());
    }

    #[test]
    fn attention_rank_keeps_uncertain_active_above_fresh_clear() {
        assert!(
            PresentationState::ActiveLastKnown.attention_rank()
                > PresentationState::ClearedCurrent.attention_rank()
        );
        assert!(
            PresentationState::ClearNeedsRefresh.attention_rank()
                > PresentationState::ClearedCurrent.attention_rank()
        );
    }
    #[test]
    fn every_state_freshness_combination_preserves_fail_safe_contract() {
        let states = [
            AlertState::Unknown,
            AlertState::Active,
            AlertState::Cleared,
        ];
        let freshnesses = [
            SourceFreshness::Unknown,
            SourceFreshness::Fresh,
            SourceFreshness::Stale,
            SourceFreshness::Unavailable,
        ];
        let mut checked = 0;
        for state in states {
            for freshness in freshnesses {
                let actual = presentation_state(&status(state, freshness));
                let expected = match (state, freshness) {
                    (AlertState::Unknown, _) => PresentationState::Unknown,
                    (AlertState::Active, SourceFreshness::Fresh) => PresentationState::ActiveCurrent,
                    (AlertState::Active, _) => PresentationState::ActiveLastKnown,
                    (AlertState::Cleared, SourceFreshness::Fresh) => PresentationState::ClearedCurrent,
                    (AlertState::Cleared, _) => PresentationState::ClearNeedsRefresh,
                };
                assert_eq!(actual, expected, "state={state:?}, freshness={freshness:?}");
                assert_eq!(
                    actual.may_show_as_safe(),
                    state == AlertState::Cleared && freshness == SourceFreshness::Fresh,
                    "safe gate: state={state:?}, freshness={freshness:?}"
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 12);
    }

    #[test]
    fn previously_fresh_clear_becomes_unconfirmed_when_source_degrades() {
        let mut cleared = status(AlertState::Cleared, SourceFreshness::Fresh);
        assert_eq!(presentation_state(&cleared), PresentationState::ClearedCurrent);
        for freshness in [
            SourceFreshness::Stale,
            SourceFreshness::Unavailable,
            SourceFreshness::Unknown,
        ] {
            cleared.set_source_freshness(freshness);
            let visible = presentation_state(&cleared);
            assert_eq!(visible, PresentationState::ClearNeedsRefresh);
            assert!(!visible.may_show_as_safe());
        }
    }

}
