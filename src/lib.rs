#![type_length_limit = "94603681"]
//! A library for parsing RTMP messages.

#![deny(
    anonymous_parameters,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // Temporarily removed, this has a false-positive on `Reference`
    //unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences,
    // missing_docs
)]


pub(crate) mod nom_utils;
mod errors;
pub(crate) mod handshake;
pub(crate) mod chunk;
mod rtmp;
