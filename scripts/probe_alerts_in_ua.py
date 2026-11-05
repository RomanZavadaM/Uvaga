#!/usr/bin/env python3
"""Server-side diagnostic intake ONLY; not an emergency alert service.

No HTTP unless explicitly invoked with --live and a server-side environment token.
Never send credentials to desktop users or logs. No production deployment.
"""
import argparse
import json
import os
import sys
import urllib.error
import urllib.request

URL = "https://api.alerts.in.ua/v1/alerts/active.json"
MAX_BYTES = 2_000_000

def receive(fetcher, token: str):
    request = urllib.request.Request(
        URL,
        headers={"Authorization": "Bearer " + token, "Accept": "application/json", "User-Agent": "Uvaga-ingest-probe/0.1"},
    )
    with fetcher(request, timeout=12) as response:
        data = response.read(MAX_BYTES + 1)
        if len(data) > MAX_BYTES:
            raise ValueError("oversized source payload")
        parsed = json.loads(data)
        if not isinstance(parsed, (dict, list)):
            raise ValueError("unexpected JSON top-level")
        return {"source": "alerts.in.ua", "transport": "https", "records_type": type(parsed).__name__,
                "count": len(parsed), "admitted_official": False}

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--live", action="store_true", help="explicit one-shot authenticated request; requires owner-approved source access")
    args = parser.parse_args()
    if not args.live:
        print("DRY RUN: no network request; set --live and UVAGA_ALERTS_IN_UA_TOKEN on an approved server")
        return 0
    token = os.environ.get("UVAGA_ALERTS_IN_UA_TOKEN", "").strip()
    if not token:
        print("ERROR: token not provided in server environment; no network request", file=sys.stderr)
        return 2
    try:
        print(json.dumps(receive(urllib.request.urlopen, token), ensure_ascii=False))
        return 0
    except (urllib.error.URLError, ValueError, json.JSONDecodeError) as exc:
        print("ERROR: source not available or invalid response (" + type(exc).__name__ + ")", file=sys.stderr)
        return 1

if __name__ == "__main__":
    raise SystemExit(main())
