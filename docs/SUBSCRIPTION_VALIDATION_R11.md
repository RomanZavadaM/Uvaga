# Subscription catalog validation r11

Reason: r9 allowed any arbitrary TerritoryKey to be stored as a subscription. That could create silent configuration errors and exact-match an invalid code.

## Change
- subscribe() now requires the current TerritoryCatalog.
- Unknown territories return SubscriptionError::UnknownTerritory.
- Event matching also returns false for an event territory absent from the catalog.
- Existing hierarchy semantics remain unchanged.

## Predeclared acceptance
1. Known territory subscriptions still match correctly.
2. Duplicate known subscription remains idempotent.
3. Unknown subscription is rejected and not stored.
4. Unknown event territory does not match.
5. All earlier source/territory tests remain green.

This is still local in-memory selection logic: no user account, geolocation, persistence, push token or public delivery.
