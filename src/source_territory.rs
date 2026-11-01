use crate::{
    source::{normalize_official, NormalizeError, SourceObservation},
    territory::TerritoryKey,
    territory_mapping::TerritoryMappingRegistry,
    OfficialEvent,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MappedOfficialEvent {
    pub event: OfficialEvent,
    pub territory: TerritoryKey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceTerritoryError {
    UnmappedTerritory,
    Source(NormalizeError),
}

pub fn normalize_official_mapped(
    observation: &SourceObservation,
    mappings: &TerritoryMappingRegistry,
) -> Result<MappedOfficialEvent, SourceTerritoryError> {
    let target = mappings
        .resolve(&observation.source_id, &observation.region)
        .cloned()
        .ok_or(SourceTerritoryError::UnmappedTerritory)?;

    let mut mapped = observation.clone();
    mapped.region = target.code.clone();

    let event = normalize_official(&mapped).map_err(SourceTerritoryError::Source)?;

    Ok(MappedOfficialEvent {
        event,
        territory: target,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        source::{ObservationKind, SourceClass},
        territory::{TerritoryCatalog, TerritoryLevel, TerritoryNode},
        territory_mapping::ProviderTerritoryKey,
        Signal,
    };

    fn key(level: TerritoryLevel, code: &str) -> TerritoryKey {
        TerritoryKey {
            level,
            code: code.into(),
        }
    }

    fn catalog_and_mappings() -> (TerritoryCatalog, TerritoryMappingRegistry) {
        let mut catalog = TerritoryCatalog::default();
        let ua = key(TerritoryLevel::Country, "UA");
        catalog
            .insert(TerritoryNode {
                key: ua.clone(),
                name_uk: "Україна".into(),
                parent: None,
            })
            .unwrap();

        let lviv = key(TerritoryLevel::Region, "UA-46");
        catalog
            .insert(TerritoryNode {
                key: lviv.clone(),
                name_uk: "Львівська область".into(),
                parent: Some(ua),
            })
            .unwrap();

        let mut mappings = TerritoryMappingRegistry::default();
        mappings
            .insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "official-api".into(),
                    provider_code: "provider-lviv".into(),
                },
                lviv,
            )
            .unwrap();

        (catalog, mappings)
    }

    fn observation() -> SourceObservation {
        SourceObservation {
            source_id: "official-api".into(),
            source_class: SourceClass::Official,
            provider_event_id: "evt-1".into(),
            region: "provider-lviv".into(),
            issued_at: 100,
            observed_at: 101,
            kind: ObservationKind::OfficialStart,
            trusted_adapter: true,
        }
    }

    #[test]
    fn known_provider_region_normalizes_to_core_territory() {
        let (_, mappings) = catalog_and_mappings();
        let mapped = normalize_official_mapped(&observation(), &mappings).unwrap();

        assert_eq!(mapped.territory, key(TerritoryLevel::Region, "UA-46"));
        assert_eq!(mapped.event.region, "UA-46");
        assert_eq!(mapped.event.signal, Signal::Start);
    }

    #[test]
    fn unmapped_provider_region_fails_closed() {
        let (_, mappings) = catalog_and_mappings();
        let mut o = observation();
        o.region = "unknown-provider-region".into();

        assert_eq!(
            normalize_official_mapped(&o, &mappings),
            Err(SourceTerritoryError::UnmappedTerritory)
        );
    }

    #[test]
    fn mapping_does_not_bypass_source_trust() {
        let (_, mappings) = catalog_and_mappings();
        let mut o = observation();
        o.trusted_adapter = false;

        assert_eq!(
            normalize_official_mapped(&o, &mappings),
            Err(SourceTerritoryError::Source(
                NormalizeError::UntrustedAdapter
            ))
        );
    }

    #[test]
    fn aggregator_cannot_become_official_via_mapping() {
        let (_, mappings) = catalog_and_mappings();
        let mut o = observation();
        o.source_class = SourceClass::Aggregator;

        assert_eq!(
            normalize_official_mapped(&o, &mappings),
            Err(SourceTerritoryError::Source(
                NormalizeError::NotOfficialSource
            ))
        );
    }

    #[test]
    fn provider_region_string_is_not_leaked_as_core_region() {
        let (_, mappings) = catalog_and_mappings();
        let mapped = normalize_official_mapped(&observation(), &mappings).unwrap();

        assert_ne!(mapped.event.region, "provider-lviv");
        assert_eq!(mapped.event.region, mapped.territory.code);
    }

    #[test]
    fn surrounding_provider_whitespace_does_not_break_mapping_or_identity() {
        let (_, mappings) = catalog_and_mappings();
        let mut o = observation();
        o.source_id = " official-api ".into();
        o.provider_event_id = " evt-1 ".into();
        o.region = " provider-lviv ".into();

        let mapped = normalize_official_mapped(&o, &mappings).unwrap();
        assert_eq!(mapped.territory, key(TerritoryLevel::Region, "UA-46"));
        assert_eq!(mapped.event.event_id, "official-api:evt-1:100:start");
    }
}
