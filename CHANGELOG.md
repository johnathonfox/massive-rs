# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Additive `{method}_with_params` variants for every REST method with optional
  arguments, taking the required arguments positionally plus a chainable
  `{CamelName}Params` struct (owned fields, `options` included). The flat
  positional methods remain the Python-parity surface and delegate to these.
- `WebSocketControl` handle (`WebSocketClient::control()`) for live
  subscribe/unsubscribe/unsubscribe-all while `connect` is running, and
  `WebSocketClient::with_host` for testing against alternate endpoints.
- Examples: `financials`, `indicators`, `reference`.
- Integration tests for the params builders (`tests/rest_params.rs`) and for the
  WebSocket auth handshake, live reconcile, and reconnect/resubscribe against a
  local server (`tests/websocket.rs`).

### Changed

- REST traits are now object-safe: `get_*` methods return `BoxFuture<'a, T>`
  and `list_*` methods return `BoxStream<'a, T>` (aliases in `massive::rest`),
  so `Box<dyn AggsApi>` and friends work for mocking and dependency injection.
- Query serialization is serde-derived on the params structs (`Serialize`
  derive with `rename`/`skip_serializing_if` attributes) instead of
  hand-pushed `Vec<(&str, String)>` pairs; `Client::list` is now the single
  transport entry point owning the pagination branch. No public signature
  changes; wire format is unchanged (covered by per-module full-params
  wiremock tests).

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
