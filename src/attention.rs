use std::cmp::Ordering;

use crate::presentation::PresentationState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttentionClass {
    /// Official state already derived by the shared fail-safe presentation model.
    Official(PresentationState),
    /// Additional threat information admitted by the verification layer.
    VerifiedSupplementalThreat,
    /// Non-urgent verified explanatory/service information.
    Informational,
}

impl AttentionClass {
    pub fn rank(self) -> u16 {
        match self {
            // Keep official ordering anchored to PresentationState::attention_rank().
            // Multiplying leaves room to place supplemental threat between
            // ActiveCurrent (400) and ClearNeedsRefresh (300).
            Self::Official(state) => state.attention_rank() as u16 * 100,
            Self::VerifiedSupplementalThreat => 350,
            Self::Informational => 50,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttentionItem {
    pub id: String,
    pub class: AttentionClass,
    pub issued_at: i64,
}

/// Deterministic display order:
/// 1. attention class (highest first),
/// 2. newer event first,
/// 3. stable lexical ID tie-break.
///
/// This function assumes upstream verification already decided that an item may be
/// displayed. There is intentionally no "unverified rumor" class.
pub fn display_cmp(left: &AttentionItem, right: &AttentionItem) -> Ordering {
    right
        .class
        .rank()
        .cmp(&left.class.rank())
        .then_with(|| right.issued_at.cmp(&left.issued_at))
        .then_with(|| left.id.cmp(&right.id))
}

pub fn sort_for_display(items: &mut [AttentionItem]) {
    items.sort_by(display_cmp);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str, class: AttentionClass, issued_at: i64) -> AttentionItem {
        AttentionItem {
            id: id.into(),
            class,
            issued_at,
        }
    }

    #[test]
    fn official_order_is_derived_from_shared_presentation_rank() {
        let states = [
            PresentationState::ActiveLastKnown,
            PresentationState::ActiveCurrent,
            PresentationState::ClearNeedsRefresh,
            PresentationState::Unknown,
            PresentationState::ClearedCurrent,
        ];

        for left in states {
            for right in states {
                assert_eq!(
                    AttentionClass::Official(left)
                        .rank()
                        .cmp(&AttentionClass::Official(right).rank()),
                    left.attention_rank().cmp(&right.attention_rank())
                );
            }
        }
    }

    #[test]
    fn official_active_classes_stay_above_supplemental_threat() {
        assert!(
            AttentionClass::Official(PresentationState::ActiveLastKnown).rank()
                > AttentionClass::VerifiedSupplementalThreat.rank()
        );
        assert!(
            AttentionClass::Official(PresentationState::ActiveCurrent).rank()
                > AttentionClass::VerifiedSupplementalThreat.rank()
        );
    }

    #[test]
    fn supplemental_threat_sits_above_clear_needs_refresh_and_clear() {
        assert!(
            AttentionClass::VerifiedSupplementalThreat.rank()
                > AttentionClass::Official(PresentationState::ClearNeedsRefresh).rank()
        );
        assert!(
            AttentionClass::VerifiedSupplementalThreat.rank()
                > AttentionClass::Official(PresentationState::ClearedCurrent).rank()
        );
    }

    #[test]
    fn stale_or_unknown_official_state_stays_above_current_clear() {
        assert!(
            AttentionClass::Official(PresentationState::ClearNeedsRefresh).rank()
                > AttentionClass::Official(PresentationState::ClearedCurrent).rank()
        );
        assert!(
            AttentionClass::Official(PresentationState::Unknown).rank()
                > AttentionClass::Official(PresentationState::ClearedCurrent).rank()
        );
    }

    #[test]
    fn same_class_uses_newer_event_first() {
        let mut items = vec![
            item("old", AttentionClass::VerifiedSupplementalThreat, 10),
            item("new", AttentionClass::VerifiedSupplementalThreat, 20),
        ];

        sort_for_display(&mut items);
        assert_eq!(
            items.iter().map(|v| v.id.as_str()).collect::<Vec<_>>(),
            vec!["new", "old"]
        );
    }

    #[test]
    fn exact_tie_is_deterministic_by_id() {
        let mut items = vec![
            item("b", AttentionClass::Informational, 10),
            item("a", AttentionClass::Informational, 10),
        ];

        sort_for_display(&mut items);
        assert_eq!(
            items.iter().map(|v| v.id.as_str()).collect::<Vec<_>>(),
            vec!["a", "b"]
        );
    }
}
