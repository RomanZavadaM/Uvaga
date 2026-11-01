use crate::{OfficialEvent, Signal};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceClass {
    Official,
    Aggregator,
    Supplemental,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationKind {
    OfficialStart,
    OfficialClear,
    Advisory,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceObservation {
    pub source_id: String,
    pub source_class: SourceClass,
    pub provider_event_id: String,
    pub region: String,
    pub issued_at: i64,
    pub observed_at: i64,
    pub kind: ObservationKind,
    /// Set only by an admitted adapter after its credentials/source boundary was validated.
    pub trusted_adapter: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NormalizeError {
    BlankSource,
    BlankEventId,
    BlankRegion,
    NotOfficialSource,
    UntrustedAdapter,
    NotOfficialSignal,
    InvalidTimeOrder,
}

pub fn normalize_official(observation: &SourceObservation) -> Result<OfficialEvent, NormalizeError> {
    if observation.source_id.trim().is_empty() {
        return Err(NormalizeError::BlankSource);
    }
    if observation.provider_event_id.trim().is_empty() {
        return Err(NormalizeError::BlankEventId);
    }
    if observation.region.trim().is_empty() {
        return Err(NormalizeError::BlankRegion);
    }
    if observation.source_class != SourceClass::Official {
        return Err(NormalizeError::NotOfficialSource);
    }
    if !observation.trusted_adapter {
        return Err(NormalizeError::UntrustedAdapter);
    }
    if observation.observed_at < observation.issued_at {
        return Err(NormalizeError::InvalidTimeOrder);
    }

    let signal = match observation.kind {
        ObservationKind::OfficialStart => Signal::Start,
        ObservationKind::OfficialClear => Signal::Clear,
        ObservationKind::Advisory => return Err(NormalizeError::NotOfficialSignal),
    };

    Ok(OfficialEvent {
        region: observation.region.clone(),
        event_id: format!("{}:{}", observation.source_id, observation.provider_event_id),
        issued_at: observation.issued_at,
        signal,
        verified_official: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obs(kind: ObservationKind) -> SourceObservation {
        SourceObservation {
            source_id: "official-api".into(),
            source_class: SourceClass::Official,
            provider_event_id: "evt-1".into(),
            region: "UA-46".into(),
            issued_at: 100,
            observed_at: 101,
            kind,
            trusted_adapter: true,
        }
    }

    #[test]
    fn trusted_official_start_normalizes() {
        let event = normalize_official(&obs(ObservationKind::OfficialStart)).unwrap();
        assert_eq!(event.signal, Signal::Start);
        assert!(event.verified_official);
        assert_eq!(event.event_id, "official-api:evt-1");
    }

    #[test]
    fn aggregator_cannot_self_promote_to_official() {
        let mut o = obs(ObservationKind::OfficialClear);
        o.source_class = SourceClass::Aggregator;
        assert_eq!(normalize_official(&o), Err(NormalizeError::NotOfficialSource));
    }

    #[test]
    fn untrusted_adapter_cannot_create_official_event() {
        let mut o = obs(ObservationKind::OfficialClear);
        o.trusted_adapter = false;
        assert_eq!(normalize_official(&o), Err(NormalizeError::UntrustedAdapter));
    }

    #[test]
    fn advisory_never_normalizes_as_official_signal() {
        assert_eq!(
            normalize_official(&obs(ObservationKind::Advisory)),
            Err(NormalizeError::NotOfficialSignal)
        );
    }

    #[test]
    fn impossible_observation_time_is_rejected() {
        let mut o = obs(ObservationKind::OfficialStart);
        o.observed_at = 99;
        assert_eq!(normalize_official(&o), Err(NormalizeError::InvalidTimeOrder));
    }
}
