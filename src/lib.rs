#![no_std]

//! # ITS One-Time MAC Public Attestation
//!
//! Standalone `#![no_std]` crate for Wegman-Carter one-time MAC tags bound to
//! non-reproducible SSS forward/backward chains. Published attestation bundles
//! can be verified by any third party without access to the signer's private state.

extern crate alloc;

pub mod field_arith;
pub mod poly;
pub mod trapdoor;
pub mod otm;
pub mod public_attestation;

pub use public_attestation::{
    create_public_attestation, verify_public_attestation, OtmChainSigner, PublicAttestationBundle,
};
