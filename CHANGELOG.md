# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `CLAUDE.md` with crate conventions and the parity-maintenance workflow.
- GitHub Actions CI (check, clippy, test, docs).
- Env-gated live tests (`tests/live.rs`, run with `--ignored` and `MASSIVE_API_KEY`).
- Opt-in retry on HTTP 429/5xx with exponential backoff via
  `Client::with_max_retries` (default 0, preserving Python-client behavior).

## [0.1.0] - 2026-08-03

Initial implementation with full parity to the official Python client
([massive-com/client-python](https://github.com/massive-com/client-python)),
verified against commit `481e5c270ea85e8eae5e96f8b9fda34e5e2a674a` (2026-07-09).

### Added

- REST: all 14 endpoint modules (91 methods) — aggs, trades, quotes, snapshot,
  reference, financials, indicators, futures, economy, etf_global, tmx, summaries,
  benzinga, vx — as traits on `Client` with typed `Option` args, dotted filter
  operators, and automatic `next_url` pagination via `futures::Stream`.
- Models: 21 files mirroring `massive/rest/models` 1:1, serde wire names verified
  field-by-field against the Python `from_dict` mappings.
- WebSocket: `WebSocketClient` with auth handshake, live subscribe/unsubscribe
  reconciliation, reconnect with resubscribe, all feeds/markets, and market-aware
  message parsing.
- 98 wiremock integration + unit tests (no API key required).
- Examples: aggs, last_trade_quote, snapshot, websocket.
