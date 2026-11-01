# Territory subscriptions r9 — safety-oriented local selection

## Purpose
Select territorial alerts a client cares about without storing identity, device tokens, precise location or accounts.

## Safety matching rule

A subscription and an event match when their known territorial scopes **overlap in the same hierarchy branch**:

- exact territory matches;
- selecting a broad scope (for example Region) matches alerts for its descendants;
- selecting a narrow scope (for example Hromada or Locality) also matches alerts issued for its ancestors (Region/Raion/Hromada);
- a country-wide alert matches every known subscription inside the country;
- unrelated branches do not match.

This prevents a dangerous failure where a user subscribed to one community would miss a region-wide alert covering that community.

## Catalog validation
- `subscribe()` accepts only a territory already present in the accepted `TerritoryCatalog`;
- unknown subscription territory returns `SubscriptionError::UnknownTerritory`;
- unknown event territory fails closed and does not match;
- duplicate subscription is idempotent.

## Predeclared acceptance
1. Region subscription matches descendant Locality alert.
2. Locality subscription matches ancestor Region alert.
3. Hromada subscription matches ancestor Raion and Region alerts.
4. Country-wide alert matches a known local subscription.
5. Unrelated region branch does not match.
6. Exact match works.
7. Unknown subscription is rejected.
8. Unknown event territory does not match.
9. All prior core/relay tests remain green.

## Privacy / scope limits
The model is an in-memory set only. There are no accounts, geolocation, device IDs, push tokens or persistence. This matching decides territorial relevance only; it does not determine whether the underlying message is verified or safe, and it cannot change official alert state.
