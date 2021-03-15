//! Data structures to provide transformation of the source
// addresses of a WebAssembly module into the native code.

use crate::lib::std::vec::Vec;
use crate::sourceloc::SourceLoc;
#[cfg(feature = "enable-borsh")]
use borsh::{BorshDeserialize, BorshSerialize};
#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};
use rkyv::{Archive, Deserialize as rkyvDe, Serialize as rkyvSer};

/// Single source location to generated address mapping.
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "enable-borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Archive, rkyvDe, rkyvSer)]
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
#[cfg_attr(feature = "enable-borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default, Archive, rkyvDe, rkyvSer)]
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
