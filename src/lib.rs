//! Async Rust client for the Massive.com (formerly Polygon.io) REST and WebSocket API.
//!
//! Feature parity target: the official Python client
//! (<https://github.com/massive-com/client-python>).

// The per-module API traits use `async fn` by design (mirroring the Python client's
// method-per-endpoint surface); they are only implemented for `Client`.
#![allow(async_fn_in_trait)]
// Flat positional `Option` args mirror the Python client's kwargs by design.
#![allow(clippy::too_many_arguments)]

pub mod client;
pub mod error;
pub mod models;
pub mod paginate;
pub mod rest;
pub mod websocket;

pub use client::{Client, RequestOptions};
pub use error::{Error, Result};
pub use rest::*;

/// The default REST client, equivalent to the Python client's `RESTClient`.
pub type RESTClient = Client;
