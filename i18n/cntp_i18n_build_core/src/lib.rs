//! # `cntp_i18n_build-core`
//!
//! This crate provides shared build-time utilities for the cntp_i18n system.
//! It is used internally by `cntp_i18n_gen` and `cntp_i18n_macros` to handle configuration
//! loading and translation file parsing.
//!
//! ## Overview
//!
//! Most users will not need to use this crate directly. Instead, use:
//! - `cntp_i18n` - The main runtime crate with macros
//! - `cntp_i18n_gen` - For build script integration
//!
//! ## Modules
//!
//! - [`config`] - Configuration file (`i18n.toml`) loading and parsing
//! - [`load`] - Translation file (`.json`) loading utilities

#![warn(missing_docs)]

pub mod config;
pub mod load;
