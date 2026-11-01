use std::collections::BTreeMap;

use crate::territory::{TerritoryCatalog, TerritoryKey};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProviderTerritoryKey {
    pub source_id: String,
    pub provider_code: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MappingError {
    BlankSource,
    BlankProviderCode,
    UnknownTerritory,
    DuplicateProviderKey,
}

#[derive(Clone, Debug, Default)]
pub struct TerritoryMappingRegistry {
    mappings: BTreeMap<ProviderTerritoryKey, TerritoryKey>,
}

impl TerritoryMappingRegistry {
    pub fn insert(
        &mut self,
        catalog: &TerritoryCatalog,
        provider: ProviderTerritoryKey,
        target: TerritoryKey,
    ) -> Result<(), MappingError> {
        if provider.source_id.trim().is_empty() {
            return Err(MappingError::BlankSource);
        }
        if provider.provider_code.trim().is_empty() {
            return Err(MappingError::BlankProviderCode);
        }
        if catalog.get(&target).is_none() {
            return Err(MappingError::UnknownTerritory);
        }
        if self.mappings.contains_key(&provider) {
            return Err(MappingError::DuplicateProviderKey);
        }

        self.mappings.insert(provider, target);
        Ok(())
    }

    pub fn resolve(&self, source_id: &str, provider_code: &str) -> Option<&TerritoryKey> {
        self.mappings.get(&ProviderTerritoryKey {
            source_id: source_id.to_owned(),
            provider_code: provider_code.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::territory::{
        TerritoryLevel, TerritoryNode,
    };

    fn key(level: TerritoryLevel, code: &str) -> TerritoryKey {
        TerritoryKey {
            level,
            code: code.into(),
        }
    }

    fn node(
        level: TerritoryLevel,
        code: &str,
        name: &str,
        parent: Option<TerritoryKey>,
    ) -> TerritoryNode {
        TerritoryNode {
            key: key(level, code),
            name_uk: name.into(),
            parent,
        }
    }

    fn catalog() -> TerritoryCatalog {
        let mut catalog = TerritoryCatalog::default();
        let ua = key(TerritoryLevel::Country, "UA");
        catalog
            .insert(node(TerritoryLevel::Country, "UA", "Україна", None))
            .unwrap();
        catalog
            .insert(node(
                TerritoryLevel::Oblast,
                "UA-46",
                "Львівська область",
                Some(ua),
            ))
            .unwrap();
        catalog
    }

    #[test]
    fn maps_provider_code_to_source_neutral_territory() {
        let catalog = catalog();
        let mut mappings = TerritoryMappingRegistry::default();

        assert_eq!(
            mappings.insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "alerts-in-ua".into(),
                    provider_code: "27".into(),
                },
                key(TerritoryLevel::Oblast, "UA-46"),
            ),
            Ok(())
        );

        assert_eq!(
            mappings.resolve("alerts-in-ua", "27"),
            Some(&key(TerritoryLevel::Oblast, "UA-46"))
        );
    }

    #[test]
    fn same_provider_code_can_mean_different_things_for_different_sources() {
        let catalog = catalog();
        let mut mappings = TerritoryMappingRegistry::default();

        mappings
            .insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "source-a".into(),
                    provider_code: "46".into(),
                },
                key(TerritoryLevel::Oblast, "UA-46"),
            )
            .unwrap();

        assert!(mappings.resolve("source-b", "46").is_none());
    }

    #[test]
    fn rejects_mapping_to_unknown_core_territory() {
        let catalog = catalog();
        let mut mappings = TerritoryMappingRegistry::default();

        assert_eq!(
            mappings.insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "source-a".into(),
                    provider_code: "missing".into(),
                },
                key(TerritoryLevel::Oblast, "UA-99"),
            ),
            Err(MappingError::UnknownTerritory)
        );
    }

    #[test]
    fn duplicate_provider_mapping_fails_closed() {
        let catalog = catalog();
        let mut mappings = TerritoryMappingRegistry::default();
        let provider = ProviderTerritoryKey {
            source_id: "source-a".into(),
            provider_code: "46".into(),
        };

        mappings
            .insert(
                &catalog,
                provider.clone(),
                key(TerritoryLevel::Oblast, "UA-46"),
            )
            .unwrap();

        assert_eq!(
            mappings.insert(
                &catalog,
                provider,
                key(TerritoryLevel::Oblast, "UA-46"),
            ),
            Err(MappingError::DuplicateProviderKey)
        );
    }

    #[test]
    fn blank_provenance_is_rejected() {
        let catalog = catalog();
        let mut mappings = TerritoryMappingRegistry::default();

        assert_eq!(
            mappings.insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: " ".into(),
                    provider_code: "46".into(),
                },
                key(TerritoryLevel::Oblast, "UA-46"),
            ),
            Err(MappingError::BlankSource)
        );
        assert_eq!(
            mappings.insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "source-a".into(),
                    provider_code: "".into(),
                },
                key(TerritoryLevel::Oblast, "UA-46"),
            ),
            Err(MappingError::BlankProviderCode)
        );
    }
}
