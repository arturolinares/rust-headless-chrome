#![cfg_attr(feature = "nightly", feature(external_doc))]
#![deny(clippy::pedantic)]
#![allow(
    clippy::stutter,
    clippy::doc_markdown, // a number of false positives here
    clippy::default_trait_access, // fails on output of derive_builder
    clippy::needless_pass_by_value // would stop us creating and passing in LaunchOptions to browser in one statement
)]

extern crate log;

#[macro_use]
extern crate derive_builder;

pub mod browser;
pub mod cdtp;

pub use browser::{Browser, LaunchOptionsBuilder, Tab};

#[cfg(feature = "nightly")]
#[doc(include = "../README.md")]
#[allow(dead_code)]
type _READMETEST = ();
