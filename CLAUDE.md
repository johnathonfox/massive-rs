# CLAUDE.md

Async Rust client for the Massive.com (formerly Polygon.io) REST and WebSocket APIs.
Feature-parity target: the official Python client
<https://github.com/massive-com/client-python>.

## Commands

- `cargo check` — fast compile check
- `cargo clippy --all-targets -- -D warnings` — must stay clean (CI enforces)
- `cargo test` — full wiremock-based suite, no API key needed
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` — docs must build warning-free
- Live tests (real API): `MASSIVE_API_KEY=... cargo test --test live -- --ignored`

## Layout and conventions

- `src/client.rs` — `Client` (alias `RESTClient`): base URL, auth headers, pagination
  toggle, trace, opt-in retries (`with_max_retries`). Crate-internal helpers:
  `get`, `paginate`, `single_page`.
- `src/paginate.rs` — `PaginatedStream` follows `next_url`; every page request carries
  the auth/edge headers; `send_with_retry` (429/5xx, exponential backoff) is shared
  by `get` and the stream.
- `src/rest/<module>.rs` — one `pub trait XxxApi` + `impl XxxApi for Client` per
  Python `massive/rest/<module>.py`. Method names, paths, and parameter order mirror
  Python 1:1.
- `src/models/<module>.rs` — serde structs mirroring `massive/rest/models/<module>.py`.
- `src/websocket/` — `WebSocketClient` (auth, reconcile, reconnect) + market-aware
  message parsing.
- `tests/rest_<module>.rs` — wiremock integration tests, one per REST module.

## API style rules (do not deviate)

- One method per endpoint; flat positional args in Python signature order.
- Python `str`/date/enum params → `&str`/`Option<&str>`; `int` → `i64`;
  `float` → `f64`; `bool` → `Option<bool>`. Python `raw`/`params` escape hatches
  are intentionally omitted. `options: Option<&RequestOptions>` is always last.
- Filter operators are separate args serialized with dotted keys:
  `ticker_gte` → `"ticker.gte"`, `tickers_any_of` → `"tickers.any_of"`.
- `list_*` → `impl Stream<Item = Result<T>>` via `self.paginate`/`self.single_page`
  (branch on `self.pagination`). `get_*` → `async fn -> Result<T>` via `self.get`,
  unwrapping the Python `result_key` with a local `Resp` struct.
- Query params: `Vec<(&str, String)>`, pushed only when `Some`. No client-side
  defaults — `None` means the param is omitted (server defaults apply).
- Models: all fields `Option<...>` unless Python declares them required; serde
  renames taken from each Python class's `from_dict` wire keys exactly (some are
  short keys like `"sym"`, some camelCase, some snake_case — check each).
- Crate-level allows in `src/lib.rs` (`async_fn_in_trait`, `too_many_arguments`)
  are deliberate; keep the style that requires them.

## Parity maintenance workflow

Parity was verified field-by-field against Python client commit
`481e5c270ea85e8eae5e96f8b9fda34e5e2a674a` (2026-07-09). When the Python client
changes:

1. Clone/update it: `git clone https://github.com/massive-com/client-python /tmp/massive-client-python`
2. Diff `massive/rest/*.py` and `massive/rest/models/*.py` from the snapshot commit.
3. Port changes into the matching `src/rest/`/`src/models/` files using the rules
   above; add/extend wiremock tests for any new endpoint or changed payload shape.
4. Update the snapshot hash in this file and in `CHANGELOG.md`.

Known intentional deviations: no `raw`/`params` args; no `WebSocketClient.run`;
`RealTimeCurrencyConversion.from_` keeps the Python wire-key quirk (`from_`);
retries are opt-in (Python client has none).
