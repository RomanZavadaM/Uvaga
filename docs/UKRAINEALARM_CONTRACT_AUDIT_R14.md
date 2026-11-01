# UkraineAlarm API contract audit r14

Date checked: 2026-11-01  
Public API request page: https://api.ukrainealarm.com/  
Public offer currently linked by that page: https://api.ukrainealarm.com/contract.pdf

The linked PDF is titled as a 2023 public offer but was still the contract linked from the live API request page when rechecked on 2026-11-01.

This is an operational/technical reading, not legal advice and not acceptance of the offer. No API application or credential was submitted by this project in this stage.

## Confirmed points relevant to architecture

- Access requires an application and provider approval.
- The public request form requires agreement with the offer.
- Registration/application plus receipt of an API key is treated by the offer as acceptance.
- The service supports HTTP API requests and may also send webhook callbacks to a public service address supplied by the customer.
- An API key uniquely identifies customer requests.
- Notification protocol formats/content/types may change. The provider describes notice for some incompatible changes, while other changes may be introduced with different/no notice.
- Access may be refused or stopped by the provider.
- Use is free under the currently linked offer.
- The offer allows use for own purposes including commercial ones, subject to its restrictions, including restrictions on duplicating the API for commercial gain and on reverse engineering/derivative use of the API itself without permission.
- The service is provided “as is”; accuracy, timeliness, uninterrupted operation, safe/error-free operation and successful delivery are not guaranteed.
- The offer disclaims responsibility for delayed, missing, false or technically undeliverable notifications.
- The offer can be changed by publishing a new version on the provider site; continued use after publication is described as acceptance of the new terms.

## Project consequences

1. **No client-side key.** Desktop/mobile/web clients never contain the UkraineAlarm API key. A controlled relay owns credentials.
2. **Source outage is a first-class state.** API/webhook silence, revoked access, timeout, authentication failure or format failure never becomes “clear”.
3. **Polling and webhook are adapters, not core truth.** Both normalize into the same source contract and event ordering rules.
4. **Schema changes fail closed.** Unknown/incompatible payloads are quarantined/logged as source failure; they do not mutate the last confirmed official state.
5. **Do not make availability claims stronger than the provider.** The product supplements, not replaces, official public warning channels.
6. **Adapter contract/version is recorded.** Production adapter records expected provider schema/version/field assumptions and fixture tests.
7. **Credentials and webhook secrets stay outside GitHub and Drive mirror.**
8. **Rate/retry behavior must be bounded.** An unavailable provider is not hammered indefinitely; backoff and source-health state are required before live integration.
9. **Contract drift is monitored.** Before first live use and before relevant releases, re-open the provider request page/offer, record the checked version/date and compare material terms.
10. **Owner approval required before application/offer acceptance.** Submitting the API request, receiving/using credentials or accepting changed terms is outside the current coding authorization.

## Preconditions before live adapter work

- owner explicitly approves API application/offer acceptance;
- current live offer is rechecked and the check is recorded;
- actual credentials are stored in an approved secret store;
- sample/sandbox or recorded real payloads are obtained lawfully;
- adapter tests cover start, clear, reused provider record IDs, duplicates, delayed/reordered messages, malformed schema, authentication failure, provider outage and webhook retry;
- verification confirms there is no route from parse/transport/authentication errors to an official clear;
- source disable switch and rollback behavior are tested;
- client applications can continue to display last-confirmed state + source freshness when relay/provider access is unavailable.

Until those conditions are met, UkraineAlarm integration remains documentation + offline fixture work only.
