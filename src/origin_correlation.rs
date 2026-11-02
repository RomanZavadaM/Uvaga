//! Synthetic source-origin correlation. Does not ingest real alerts.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntakeReport {
    pub transport: String,
    /// Original issuer/event identity, not the forwarding platform's identity.
    pub origin: String,
    pub origin_event: String,
    pub territory: String,
    pub kind: String,
    pub issued_at: i64,
    pub authenticated: bool,
    pub competent_for_kind: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Correlation {
    pub distinct_origins: usize,
    pub transmissions: usize,
    pub accepted_origins: usize,
    pub suspect_origins: usize,
    pub summary: IntakeVerdict,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntakeVerdict { NoEvidence, Unverified, Verified, Conflicting }

/// Same-origin forwards never become independent confirmations.
/// This is intake evidence only; it cannot issue or clear an official signal.
pub fn correlate(reports: &[IntakeReport], territory: &str, kind: &str) -> Correlation {
    let mut by_origin: BTreeMap<&str, Vec<&IntakeReport>> = BTreeMap::new();
    let mut transmissions = 0;
    for report in reports.iter().filter(|r| r.territory == territory && r.kind == kind) {
        transmissions += 1;
        by_origin.entry(&report.origin).or_default().push(report);
    }
    let mut accepted = 0;
    let mut suspect = 0;
    let mut fingerprints = BTreeSet::new();
    for (origin, copies) in &by_origin {
        let identities: BTreeSet<_> = copies.iter().map(|r| (r.origin_event.as_str(), r.issued_at)).collect();
        let trusted = !origin.trim().is_empty()
            && identities.len() == 1
            && copies.iter().all(|r| r.authenticated && r.competent_for_kind && !r.origin_event.trim().is_empty());
        if trusted {
            accepted += 1;
            fingerprints.insert(*identities.iter().next().unwrap());
        } else {
            suspect += 1;
        }
    }
    let summary = if by_origin.is_empty() { IntakeVerdict::NoEvidence }
        else if suspect > 0 { IntakeVerdict::Unverified }
        else if fingerprints.len() > 1 { IntakeVerdict::Conflicting }
        else { IntakeVerdict::Verified };
    Correlation { distinct_origins: by_origin.len(), transmissions, accepted_origins: accepted, suspect_origins: suspect, summary }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn report(transport: &str, origin: &str, event: &str, authenticated: bool) -> IntakeReport {
        IntakeReport { transport:transport.into(), origin:origin.into(), origin_event:event.into(), territory:"Lviv".into(), kind:"civil-warning".into(), issued_at:100, authenticated, competent_for_kind:true }
    }
    #[test]
    fn compromised_source_forwarded_by_aggregator_is_not_two_confirmations() {
        let a = report("provider", "origin-A", "event-1", false);
        let b = report("aggregator", "origin-A", "event-1", false);
        let out = correlate(&[a,b], "Lviv", "civil-warning");
        assert_eq!(out.transmissions, 2);
        assert_eq!(out.distinct_origins, 1);
        assert_eq!(out.accepted_origins, 0);
        assert_eq!(out.summary, IntakeVerdict::Unverified);
    }
    #[test]
    fn independent_admitted_source_cannot_launder_compromised_origin() {
        let reports = [report("provider", "origin-A", "event-1", false), report("aggregator", "origin-A", "event-1", false),report("official-independent", "origin-C", "event-2", true)];
        let result = correlate(&reports,"Lviv","civil-warning");
        assert_eq!(result.distinct_origins, 2);
        assert_eq!(result.accepted_origins, 1);
        assert_eq!(result.suspect_origins, 1);
        assert_eq!(result.summary, IntakeVerdict::Unverified);
    }
    #[test]
    fn conflicting_facts_from_same_origin_are_suspect() {
        let reports=[report("api","origin-A","event-1",true),report("mirror","origin-A","event-other",true)];
        assert_eq!(correlate(&reports,"Lviv","civil-warning").summary,IntakeVerdict::Unverified);
    }
    #[test]
    fn separate_territory_never_correlates_into_lviv() {
        let mut other=report("api","origin-A","event-1",true);
        other.territory="Kyiv".into();
        assert_eq!(correlate(&[other],"Lviv","civil-warning").summary,IntakeVerdict::NoEvidence);
    }
}
