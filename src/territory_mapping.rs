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

impl ProviderTerritoryKey {
    fn canonicalized(self) -> Result<Self, MappingError> {
        let source_id = self.source_id.trim();
        if source_id.is_empty() {
            return Err(MappingError::BlankSource);
        }

        let provider_code = self.provider_code.trim();
        if provider_code.is_empty() {
            return Err(MappingError::BlankProviderCode);
        }

        Ok(Self {
            source_id: source_id.to_owned(),
            provider_code: provider_code.to_owned(),
        })
    }

    fn from_lookup(source_id: &str, provider_code: &str) -> Option<Self> {
        let source_id = source_id.trim();
        let provider_code = provider_code.trim();
        if source_id.is_empty() || provider_code.is_empty() {
            return None;
        }

        Some(Self {
            source_id: source_id.to_owned(),
            provider_code: provider_code.to_owned(),
        })
    }
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
        let provider = provider.canonicalized()?;

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
        let key = ProviderTerritoryKey::from_lookup(source_id, provider_code)?;
        self.mappings.get(&key)
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
    use crate::territory::{TerritoryLevel, TerritoryNode};

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
                TerritoryLevel::Region,
                "UA-46",
                "Львівська область",
                Some(ua),
            ))
            .unwrap();
        catalog
    }

    #[test]
    fn maps_provider_code_to_source_neutral_region() {
        let catalog = catalog();
        let mut mappings = TerritoryMappingRegistry::default();

        assert_eq!(
            mappings.insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "alerts-in-ua".into(),
                    provider_code: "27".into(),
                },
                key(TerritoryLevel::Region, "UA-46"),
            ),
            Ok(())
        );

        assert_eq!(
            mappings.resolve("alerts-in-ua", "27"),
            Some(&key(TerritoryLevel::Region, "UA-46"))
        );
    }

    #[test]
    fn surrounding_whitespace_is_canonicalized_for_insert_and_lookup() {
        let catalog = catalog();
        let mut mappings = TerritoryMappingRegistry::default();

        mappings
            .insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "  source-a ".into(),
                    provider_code: " 46 ".into(),
                },
                key(TerritoryLevel::Region, "UA-46"),
            )
            .unwrap();

        assert_eq!(
            mappings.resolve(" source-a ", "46"),
            Some(&key(TerritoryLevel::Region, "UA-46"))
        );
        assert_eq!(
            mappings.insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "source-a".into(),
                    provider_code: "46".into(),
                },
                key(TerritoryLevel::Region, "UA-46"),
            ),
            Err(MappingError::DuplicateProviderKey)
        );
    }

    #[test]
    fn same_provider_code_from_different_sources_stays_independent() {
        let catalog = catalog();
        let mut mappings = TerritoryMappingRegistry::default();

        mappings
            .insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "source-a".into(),
                    provider_code: "46".into(),
                },
                key(TerritoryLevel::Region, "UA-46"),
            )
            .unwrap();

        assert!(mappings.resolve("source-b", "46").is_none());
    }

    #[test]
    fn source_and_provider_code_remain_case_sensitive() {
        let catalog = catalog();
        let mut mappings = TerritoryMappingRegistry::default();

        mappings
            .insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "Source-A".into(),
                    provider_code: "AbC".into(),
                },
                key(TerritoryLevel::Region, "UA-46"),
            )
            .unwrap();

        assert!(mappings.resolve("source-a", "AbC").is_none());
        assert!(mappings.resolve("Source-A", "abc").is_none());
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
                key(TerritoryLevel::Region, "UA-99"),
            ),
            Err(MappingError::UnknownTerritory)
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
                key(TerritoryLevel::Region, "UA-46"),
            ),
            Err(MappingError::BlankSource)
        );
        assert_eq!(
            mappings.insert(
                &catalog,
                ProviderTerritoryKey {
                    source_id: "source-a".into(),
                    provider_code: "  ".into(),
                },
                key(TerritoryLevel::Region, "UA-46"),
            ),
            Err(MappingError::BlankProviderCode)
        );

        assert!(mappings.resolve(" ", "46").is_none());
        assert!(mappings.resolve("source-a", " ").is_none());
    }
}
