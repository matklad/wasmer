use crate::CodeOffset;
use borsh::{BorshDeserialize, BorshSerialize};
#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};
use wasmer_vm::TrapCode;
use rkyv::{Archive, Deserialize as rkyvDe, Serialize as rkyvSer};

/// Information about trap.
#[cfg_attr(feature = "enable-serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "enable-borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Archive, rkyvDe, rkyvSer)]
pub struct TrapInformation {
    /// The offset of the trapping instruction in native code. It is relative to the beginning of the function.
    pub code_offset: CodeOffset,
    /// Code of the trap.
    pub trap_code: TrapCode,
}
