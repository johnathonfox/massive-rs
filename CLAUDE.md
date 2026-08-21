# CLAUDE.md

Async Rust client for the Massive.com (formerly Polygon.io) REST and WebSocket APIs.
Feature-parity target: the official Python client
<https://github.com/massive-com/client-python>.

## Commands

- `cargo check` — fast compile check
- `cargo clippy --all-targets -- -D warnings` — must stay clean (CI enforces)
- `cargo test` — full wiremock-based suite, no API key needed
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` — docs must build warning-free
- Live tests (real API): `MASSIVE_API_KEY=... cargo test --test live -- --ignored`.
  CI runs these in the `live` job, which skips cleanly when the
  `MASSIVE_API_KEY` repo secret is not configured.

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
- `list_*` → `BoxStream<'a, T>` (= `Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>>`)
  via `self.paginate`/`self.single_page` (branch on `self.pagination`).
  `get_*` → `BoxFuture<'a, T>` (= `Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>`)
  via `self.get`, unwrapping the Python `result_key` with a local `Resp` struct.
  Both aliases live in `src/rest/mod.rs`; all trait methods take a single named
  lifetime `'a` on `&self` and every reference arg so the traits stay object-safe
  (`Box<dyn AggsApi>` etc. all work; see the proof test in `src/rest/mod.rs`).
- Every method with optional args also has an additive `{name}_with_params`
  variant taking the required args positionally plus a `{CamelName}Params` struct
  (owned fields, chainable setters, `options` included). The flat positional
  method is the Python-parity surface and simply delegates to the `_with_params`
  variant — keep them in lockstep when porting Python changes.
- Query params: `Vec<(&str, String)>`, pushed only when `Some`. No client-side
  defaults — `None` means the param is omitted (server defaults apply).
- Models: all fields `Option<...>` unless Python declares them required; serde
  renames taken from each Python class's `from_dict` wire keys exactly (some are
  short keys like `"sym"`, some camelCase, some snake_case — check each).
- Crate-level allow in `src/lib.rs` (`too_many_arguments`) is deliberate; keep the
  style that requires it. (`async_fn_in_trait` is gone: the traits desugar to
  boxed futures/streams for object safety.)
- MSRV is 1.88 (see `rust-version` in `Cargo.toml`), pinned by the locked
  dependency tree: `wiremock` 0.6.5 uses let-chains (1.88) and `idna_adapter`/
  `icu_*` declare 1.86. Verify with `cargo +1.88.0 check --all-targets`.
- `tests/websocket.rs` runs the real `WebSocketClient` against a local
  tokio-tungstenite server via `WebSocketClient::with_host` (testing/alternate
  endpoints) and covers auth handshake, live reconcile, and reconnect/resubscribe.
  Live subscribe/unsubscribe during `connect` goes through the channel-based
  `WebSocketControl` handle (`client.control()`), since `connect(&mut self)`
  holds the borrow.

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
