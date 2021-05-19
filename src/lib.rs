//! This crate provides a high level wraper around the [Square API](https://developer.squareup.com).
//!
//! Currently in an alpha stage this crate is maintained by [Cybersaur](https://www.cybersaur.io/)
//! as an Open Source Project, aimed to promote the use of Rust.
//!
//! # Usage This crate can be used by added it as a dependency in your
//! projects `Cargo.toml`.
//! ```toml
//! [dependencies]
//! square-rs = "0.1.0"
//! ```
//! # Examples
//! For examples of how this library can be used, see the `examples` directory in the repository.
//! The examples are a work in progress, although the `payments_api` example provides a good use case.
//! It can be run with `cargo run --example payments_api`.

#![deny(clippy::all)]

pub mod client;
pub mod endpoint;
pub mod error;
pub mod money;
pub mod payment;
pub mod response;
