# alerts.in.ua v2 compact parser r6

Scope: offline parser only. No token, HTTP client, polling, cache, proxy, notification or production behavior.

Source documentation verified 2026-11-01:
- https://devs.alerts.in.ua/
- https://devs.alerts.in.ua/start

The v2 compact format uses:
- R: red active alert
- Y: yellow / preliminary threat
- P: partial alert
- N: no alert information
- space: no region data / UID not assigned

For the oblast endpoint, 27 characters correspond to the documented fixed oblast/city order.

Safety boundary: parser results are aggregator observations only. There is deliberately no conversion from CompactStatus to OfficialEvent. In particular, N must not silently become an official clear.

Predeclared acceptance:
1. documented 27-character example parses;
2. R/Y/P/N/space remain distinct;
3. unknown codes fail closed;
4. oblast parser rejects wrong length;
5. no official event is emitted;
6. all existing core tests remain green.
