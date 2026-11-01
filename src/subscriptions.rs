use std::collections::BTreeSet;

use crate::territory::{TerritoryCatalog, TerritoryKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubscriptionError {
    UnknownTerritory,
}

#[derive(Clone, Debug, Default)]
pub struct TerritorySubscriptions {
    selected: BTreeSet<TerritoryKey>,
}

impl TerritorySubscriptions {
    /// Subscribe only to a territory present in the accepted catalog.
    /// Ok(true) means inserted; Ok(false) means it was already selected.
    pub fn subscribe(
        &mut self,
        catalog: &TerritoryCatalog,
        key: TerritoryKey,
    ) -> Result<bool, SubscriptionError> {
        if catalog.get(&key).is_none() {
            return Err(SubscriptionError::UnknownTerritory);
        }
        Ok(self.selected.insert(key))
    }

    pub fn unsubscribe(&mut self, key: &TerritoryKey) -> bool {
        self.selected.remove(key)
    }

    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }

    pub fn len(&self) -> usize {
        self.selected.len()
    }

    /// Match when the selected scope and event scope overlap in the same hierarchy branch.
    ///
    /// Examples:
    /// - selected Region matches an event for a descendant Hromada/Locality;
    /// - selected Hromada/Locality matches an event issued for its ancestor Region/Raion;
    /// - unrelated branches do not match.
    ///
    /// Unknown event territories fail closed.
    pub fn matches(&self, catalog: &TerritoryCatalog, event_territory: &TerritoryKey) -> bool {
        if catalog.get(event_territory).is_none() {
            return false;
        }

        self.selected.iter().any(|selected| {
            selected == event_territory
                || catalog.is_descendant_of(event_territory, selected)
                || catalog.is_descendant_of(selected, event_territory)
        })
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
        let lviv = key(TerritoryLevel::Region, "UA-46");
        let lviv_raion = key(TerritoryLevel::Raion, "UA-46-R01");
        let lviv_hromada = key(TerritoryLevel::Hromada, "UA-46-H001");
        let kyiv = key(TerritoryLevel::Region, "UA-30");

        catalog
            .insert(node(TerritoryLevel::Country, "UA", "Україна", None))
            .unwrap();
        catalog
            .insert(node(
                TerritoryLevel::Region,
                "UA-46",
                "Львівська область",
                Some(ua.clone()),
            ))
            .unwrap();
        catalog
            .insert(node(
                TerritoryLevel::Raion,
                "UA-46-R01",
                "Тестовий район",
                Some(lviv.clone()),
            ))
            .unwrap();
        catalog
            .insert(node(
                TerritoryLevel::Hromada,
                "UA-46-H001",
                "Тестова громада",
                Some(lviv_raion.clone()),
            ))
            .unwrap();
        catalog
            .insert(node(
                TerritoryLevel::Locality,
                "UA-46-L001",
                "Тестовий населений пункт",
                Some(lviv_hromada),
            ))
            .unwrap();
        catalog
            .insert(node(
                TerritoryLevel::Region,
                "UA-30",
                "м. Київ",
                Some(ua),
            ))
            .unwrap();
        catalog
            .insert(node(
                TerritoryLevel::Locality,
                "UA-30-L001",
                "Тестовий населений пункт Києва",
                Some(kyiv),
            ))
            .unwrap();

        catalog
    }

    #[test]
    fn selected_region_matches_descendant_locality_alert() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Region, "UA-46"))
            .unwrap();

        assert!(subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Locality, "UA-46-L001")
        ));
    }

    #[test]
    fn selected_locality_matches_ancestor_region_alert() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Locality, "UA-46-L001"))
            .unwrap();

        assert!(subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Region, "UA-46")
        ));
    }

    #[test]
    fn selected_hromada_matches_ancestor_raion_and_region_alerts() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Hromada, "UA-46-H001"))
            .unwrap();

        assert!(subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Raion, "UA-46-R01")
        ));
        assert!(subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Region, "UA-46")
        ));
    }

    #[test]
    fn country_wide_alert_matches_known_local_subscription() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Locality, "UA-46-L001"))
            .unwrap();

        assert!(subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Country, "UA")
        ));
    }

    #[test]
    fn unrelated_region_branch_does_not_match() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Locality, "UA-46-L001"))
            .unwrap();

        assert!(!subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Locality, "UA-30-L001")
        ));
        assert!(!subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Region, "UA-30")
        ));
    }

    #[test]
    fn exact_subscription_matches_exact_territory() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        let region = key(TerritoryLevel::Region, "UA-46");

        subscriptions.subscribe(&catalog, region.clone()).unwrap();
        assert!(subscriptions.matches(&catalog, &region));
    }

    #[test]
    fn subscriptions_are_idempotent_and_removable() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        let region = key(TerritoryLevel::Region, "UA-46");

        assert_eq!(subscriptions.subscribe(&catalog, region.clone()), Ok(true));
        assert_eq!(subscriptions.subscribe(&catalog, region.clone()), Ok(false));
        assert_eq!(subscriptions.len(), 1);
        assert!(subscriptions.unsubscribe(&region));
        assert!(subscriptions.is_empty());
    }

    #[test]
    fn unknown_subscription_is_rejected() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();

        assert_eq!(
            subscriptions.subscribe(&catalog, key(TerritoryLevel::Region, "UA-99")),
            Err(SubscriptionError::UnknownTerritory)
        );
        assert!(subscriptions.is_empty());
    }

    #[test]
    fn unknown_event_territory_does_not_match() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();

        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Region, "UA-46"))
            .unwrap();

        assert!(!subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Region, "UA-99")
        ));
    }
}
