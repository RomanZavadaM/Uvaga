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
    /// Subscribe only to a territory that exists in the current canonical catalog.
    /// Returns Ok(true) when inserted, Ok(false) when it was already selected.
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

    /// A subscription to a broader territory matches that territory and its descendants.
    /// A subscription to a child never implicitly matches its parent.
    pub fn matches(&self, catalog: &TerritoryCatalog, event_territory: &TerritoryKey) -> bool {
        if catalog.get(event_territory).is_none() {
            return false;
        }

        self.selected.iter().any(|selected| {
            selected == event_territory || catalog.is_descendant_of(event_territory, selected)
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
        let oblast = key(TerritoryLevel::Oblast, "UA-46");
        let raion = key(TerritoryLevel::Raion, "UA-46-R01");

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
            .insert(node(
                TerritoryLevel::Raion,
                "UA-46-R01",
                "Тестовий район",
                Some(oblast),
            ))
            .unwrap();
        catalog
            .insert(node(
                TerritoryLevel::Hromada,
                "UA-46-H001",
                "Тестова громада",
                Some(raion),
            ))
            .unwrap();

        catalog
    }

    #[test]
    fn oblast_subscription_matches_descendant_hromada() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Oblast, "UA-46"))
            .unwrap();

        assert!(subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Hromada, "UA-46-H001")
        ));
    }

    #[test]
    fn country_subscription_matches_all_known_descendants() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Country, "UA"))
            .unwrap();

        assert!(subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Oblast, "UA-46")
        ));
        assert!(subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Raion, "UA-46-R01")
        ));
    }

    #[test]
    fn child_subscription_does_not_match_parent_alert() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Hromada, "UA-46-H001"))
            .unwrap();

        assert!(!subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Oblast, "UA-46")
        ));
    }

    #[test]
    fn exact_subscription_matches_exact_territory() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        let oblast = key(TerritoryLevel::Oblast, "UA-46");
        subscriptions.subscribe(&catalog, oblast.clone()).unwrap();

        assert!(subscriptions.matches(&catalog, &oblast));
    }

    #[test]
    fn subscriptions_are_idempotent_and_removable() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        let oblast = key(TerritoryLevel::Oblast, "UA-46");

        assert_eq!(subscriptions.subscribe(&catalog, oblast.clone()), Ok(true));
        assert_eq!(subscriptions.subscribe(&catalog, oblast.clone()), Ok(false));
        assert_eq!(subscriptions.len(), 1);
        assert!(subscriptions.unsubscribe(&oblast));
        assert!(subscriptions.is_empty());
    }

    #[test]
    fn unknown_subscription_is_rejected() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();

        assert_eq!(
            subscriptions.subscribe(&catalog, key(TerritoryLevel::Oblast, "UA-99")),
            Err(SubscriptionError::UnknownTerritory)
        );
        assert!(subscriptions.is_empty());
    }

    #[test]
    fn unrelated_unknown_event_territory_does_not_match() {
        let catalog = catalog();
        let mut subscriptions = TerritorySubscriptions::default();
        subscriptions
            .subscribe(&catalog, key(TerritoryLevel::Oblast, "UA-46"))
            .unwrap();

        assert!(!subscriptions.matches(
            &catalog,
            &key(TerritoryLevel::Oblast, "UA-99")
        ));
    }
}
