//! Data structures to provide transformation of the source
// addresses of a WebAssembly module into the native code.

use crate::lib::std::vec::Vec;
use crate::sourceloc::SourceLoc;
use loupe::MemoryUsage;
#[cfg(feature = "enable-rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};

/// Single source location to generated address mapping.
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, MemoryUsage)]
#[cfg_attr(
    feature = "enable-rkyv",
    derive(RkyvSerialize, RkyvDeserialize, Archive)
)]
pub struct InstructionAddressMap {
    /// Original source location.
    pub srcloc: SourceLoc,

    /// Generated instructions offset.
    pub code_offset: usize,

    /// Generated instructions length.
    pub code_len: usize,
}

/// Function and its instructions addresses mappings.
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default, MemoryUsage)]
#[cfg_attr(
    feature = "enable-rkyv",
    derive(RkyvSerialize, RkyvDeserialize, Archive)
)]
pub struct FunctionAddressMap {
    /// Instructions maps.
    /// The array is sorted by the InstructionAddressMap::code_offset field.
    pub instructions: Vec<InstructionAddressMap>,

    /// Function start source location (normally declaration).
    pub start_srcloc: SourceLoc,

    /// Function end source location.
    pub end_srcloc: SourceLoc,

    /// Generated function body offset if applicable, otherwise 0.
    pub body_offset: usize,

    /// Generated function body length.
    pub body_len: usize,
}
