//! # Resolvers Library
//!
//! This library provides the resolver interface for the Stellar attestation service.
//! The ResolverInterface trait defines the contract between the protocol and resolver
//! implementations, enabling modular business logic for attestation validation.
#![no_std]

/// Core interface definitions and types shared across all resolver implementations.
/// This module contains the `ResolverInterface` trait and common data structures
/// like `ResolverAttestationData`, `ResolverMetadata`, and standardized error types.
pub mod interface;

// Re-export core interface types
pub use interface::{
    ResolverAttestationData, ResolverError, ResolverInterface, ResolverMetadata, ResolverType,
};
