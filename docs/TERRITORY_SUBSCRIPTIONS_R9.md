# Territory subscriptions r9 — local selection logic

Purpose: select which territorial alerts a client cares about without storing personal identity, device tokens or geolocation.

## Matching rule
- Exact selected territory matches itself.
- A broader selected territory matches all known descendants in TerritoryCatalog.
- Selecting a child does not implicitly subscribe to its parent.
- Unknown/unrelated territories do not match.
- The model is an in-memory set only; persistence, accounts, device identifiers and geolocation are out of scope.

## Predeclared acceptance
1. Oblast subscription matches descendant hromada.
2. Country subscription matches known oblast/raion descendants.
3. Child subscription does not match parent-level event.
4. Exact match works.
5. Subscribe is idempotent and unsubscribe removes selection.
6. Unknown unrelated territory does not match.
7. All prior territory/core tests remain green.

## Safety and privacy limits
This model does not decide whether a message is safe to suppress. Critical nationwide or mandatory public-warning rules, if introduced later, must be separate and may override user filtering. No precise location or personal profile is collected here.
