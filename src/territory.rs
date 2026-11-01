use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TerritoryLevel {
    Country,
    /// First-level administrative region: oblast, ARC, or a city with equivalent special status.
    Region,
    Raion,
    Hromada,
    /// Settlement/locality below the administrative layers. This is not a KATOTTG category mapping.
    Locality,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TerritoryKey {
    pub level: TerritoryLevel,
    pub code: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerritoryNode {
    pub key: TerritoryKey,
    pub name_uk: String,
    pub parent: Option<TerritoryKey>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerritoryError {
    BlankCode,
    BlankName,
    InvalidCode,
    MissingParent,
    ParentLevel,
    Duplicate,
}

fn level_rank(level: TerritoryLevel) -> u8 {
    match level {
        TerritoryLevel::Country => 0,
        TerritoryLevel::Region => 1,
        TerritoryLevel::Raion => 2,
        TerritoryLevel::Hromada => 3,
        TerritoryLevel::Locality => 4,
    }
}

fn valid_code(code: &str) -> bool {
    !code.is_empty()
        && code.len() <= 64
        && code
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ':'))
}

#[derive(Clone, Debug, Default)]
pub struct TerritoryCatalog {
    nodes: BTreeMap<TerritoryKey, TerritoryNode>,
}

impl TerritoryCatalog {
    pub fn insert(&mut self, node: TerritoryNode) -> Result<(), TerritoryError> {
        if node.key.code.trim().is_empty() {
            return Err(TerritoryError::BlankCode);
        }
        if node.name_uk.trim().is_empty() {
            return Err(TerritoryError::BlankName);
        }
        if !valid_code(&node.key.code) {
            return Err(TerritoryError::InvalidCode);
        }
        if self.nodes.contains_key(&node.key) {
            return Err(TerritoryError::Duplicate);
        }

        match (&node.parent, node.key.level) {
            (None, TerritoryLevel::Country) => {}
            (None, _) => return Err(TerritoryError::MissingParent),
            (Some(_), TerritoryLevel::Country) => return Err(TerritoryError::ParentLevel),
            (Some(parent), child_level) => {
                if !self.nodes.contains_key(parent) {
                    return Err(TerritoryError::MissingParent);
                }
                if level_rank(parent.level) >= level_rank(child_level) {
                    return Err(TerritoryError::ParentLevel);
                }
            }
        }

        self.nodes.insert(node.key.clone(), node);
        Ok(())
    }

    pub fn get(&self, key: &TerritoryKey) -> Option<&TerritoryNode> {
        self.nodes.get(key)
    }

    pub fn is_descendant_of(&self, child: &TerritoryKey, ancestor: &TerritoryKey) -> bool {
        let mut current = self.nodes.get(child).and_then(|node| node.parent.clone());

        while let Some(key) = current {
            if &key == ancestor {
                return true;
            }
            current = self.nodes.get(&key).and_then(|node| node.parent.clone());
        }

        false
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn builds_country_region_raion_hromada_locality_chain() {
        let mut catalog = TerritoryCatalog::default();
        let ua = key(TerritoryLevel::Country, "UA");
        let region = key(TerritoryLevel::Region, "UA-46");
        let raion = key(TerritoryLevel::Raion, "UA-46-R01");
        let hromada = key(TerritoryLevel::Hromada, "UA-46-H001");

        assert_eq!(
            catalog.insert(node(TerritoryLevel::Country, "UA", "Україна", None)),
            Ok(())
        );
        assert_eq!(
            catalog.insert(node(
                TerritoryLevel::Region,
                "UA-46",
                "Львівська область",
                Some(ua.clone()),
            )),
            Ok(())
        );
        assert_eq!(
            catalog.insert(node(
                TerritoryLevel::Raion,
                "UA-46-R01",
                "Тестовий район",
                Some(region.clone()),
            )),
            Ok(())
        );
        assert_eq!(
            catalog.insert(node(
                TerritoryLevel::Hromada,
                "UA-46-H001",
                "Тестова громада",
                Some(raion.clone()),
            )),
            Ok(())
        );
        assert_eq!(
            catalog.insert(node(
                TerritoryLevel::Locality,
                "UA-46-L001",
                "Тестовий населений пункт",
                Some(hromada.clone()),
            )),
            Ok(())
        );

        let locality = key(TerritoryLevel::Locality, "UA-46-L001");
        assert!(catalog.is_descendant_of(&locality, &region));
        assert!(catalog.is_descendant_of(&locality, &ua));
        assert_eq!(catalog.len(), 5);
    }

    #[test]
    fn special_status_city_can_be_a_region_without_becoming_locality() {
        let mut catalog = TerritoryCatalog::default();
        let ua = key(TerritoryLevel::Country, "UA");
        catalog
            .insert(node(TerritoryLevel::Country, "UA", "Україна", None))
            .unwrap();

        let kyiv = key(TerritoryLevel::Region, "UA-30");
        assert_eq!(
            catalog.insert(node(
                TerritoryLevel::Region,
                "UA-30",
                "м. Київ",
                Some(ua.clone()),
            )),
            Ok(())
        );
        assert!(catalog.is_descendant_of(&kyiv, &ua));
    }

    #[test]
    fn rejects_orphan_non_country_node() {
        let mut catalog = TerritoryCatalog::default();
        assert_eq!(
            catalog.insert(node(
                TerritoryLevel::Region,
                "UA-46",
                "Львівська область",
                None,
            )),
            Err(TerritoryError::MissingParent)
        );
    }

    #[test]
    fn rejects_parent_at_same_or_deeper_level() {
        let mut catalog = TerritoryCatalog::default();
        let ua = key(TerritoryLevel::Country, "UA");
        catalog
            .insert(node(TerritoryLevel::Country, "UA", "Україна", None))
            .unwrap();

        let region = key(TerritoryLevel::Region, "UA-46");
        catalog
            .insert(node(
                TerritoryLevel::Region,
                "UA-46",
                "Львівська область",
                Some(ua),
            ))
            .unwrap();

        assert_eq!(
            catalog.insert(node(
                TerritoryLevel::Region,
                "UA-30",
                "м. Київ",
                Some(region),
            )),
            Err(TerritoryError::ParentLevel)
        );
    }

    #[test]
    fn rejects_invalid_codes_and_duplicate_keys() {
        let mut catalog = TerritoryCatalog::default();
        assert_eq!(
            catalog.insert(node(TerritoryLevel::Country, "UA", "Україна", None)),
            Ok(())
        );
        assert_eq!(
            catalog.insert(node(TerritoryLevel::Country, "UA", "Україна", None)),
            Err(TerritoryError::Duplicate)
        );

        let mut other = TerritoryCatalog::default();
        assert_eq!(
            other.insert(node(TerritoryLevel::Country, "UA / bad", "Україна", None)),
            Err(TerritoryError::InvalidCode)
        );
    }
}
