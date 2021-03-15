use borsh::{BorshDeserialize, BorshSerialize};
use serde::de::{Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;
use wasmer_compiler::CompiledFunctionFrameInfo;
use rkyv::{ser::{Serializer as rkyvSerializer, serializers::WriteSerializer}, archived_value,
           de::deserializers::AllocDeserializer,
Deserialize as rkyvDe};
use bincode::Options;

/// This is the unserialized verison of `CompiledFunctionFrameInfo`.
#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct UnprocessedFunctionFrameInfo {
    #[serde(with = "serde_bytes")]
    bytes: Vec<u8>,
}

impl UnprocessedFunctionFrameInfo {
    /// Converts the `UnprocessedFunctionFrameInfo` to a `CompiledFunctionFrameInfo`
    pub fn deserialize(&self) -> CompiledFunctionFrameInfo {
        // let r = flexbuffers::Reader::get_root(&self.bytes).expect("Can't deserialize the info");
        // CompiledFunctionFrameInfo::deserialize(r).expect("Can't deserialize the info")
        let now = std::time::Instant::now();
        let bytes = &self.bytes;
        let mut pos: [u8; 8] = Default::default();
        pos.copy_from_slice(&bytes[0..8]);
        let pos: u64 = u64::from_le_bytes(pos);
        let archived = unsafe { archived_value::<CompiledFunctionFrameInfo>(&bytes[8..], pos as usize )};

//        let r:CompiledFunctionFrameInfo = BorshDeserialize::deserialize(&mut self.bytes.as_ref()).expect("Can't deserialize the info");
        if archived.traps.len() == 19335000 {
            println!("{:?}", now.elapsed());
        }

        let now2 = std::time::Instant::now();
        let r: CompiledFunctionFrameInfo = rkyvDe::deserialize(archived, &mut AllocDeserializer ).unwrap();
        if archived.traps.len() == 19335000 {
            println!("{:?}", now2.elapsed());
        }
        r
    }

    /// Converts the `CompiledFunctionFrameInfo` to a `UnprocessedFunctionFrameInfo`
    pub fn serialize(processed: &CompiledFunctionFrameInfo) -> Self {
        // let mut s = flexbuffers::FlexbufferSerializer::new();
        // processed
        //     .serialize(&mut s)
        //     .expect("Can't serialize the info");
        // let bytes = s.take_buffer();
        let mut serializer = WriteSerializer::new(vec![0u8;8]);
        if processed.traps.len() == 19335 {
            let p = processed;
            let mut processed = p.clone();
            for _ in 0..999 {
                processed.traps.extend(p.traps.clone());
            }
            println!("{} {}", processed.traps.len(), processed.address_map.instructions.len());
            use std::fs::File;
            use std::io::Write;
            let mut file = File::create("/tmp/CFFI2").unwrap();
            let pos = serializer.serialize_value(&processed)
                .expect("failed to archive test");
            let mut bytes = serializer.into_inner();
            bytes[0..8].copy_from_slice(&pos.to_le_bytes());
            println!("!!! {} {}", pos, bytes.len());

//            let bytes = BorshSerialize::try_to_vec(&processed).expect("Can't serialize the info");
//            println!("!!!@@∂ {}", bytes.len());
            file.write_all(&bytes).unwrap();
            return Self { bytes }
        }
        let pos = serializer.serialize_value(processed)
            .expect("failed to archive test");
        let mut bytes = serializer.into_inner();
        bytes[0..8].copy_from_slice(&pos.to_le_bytes());
//        let bytes = BorshSerialize::try_to_vec(&processed).expect("Can't serialize the info");
        Self { bytes }
    }
}

/// We hold the frame info in two states, mainly because we want to
/// process it lazily to speed up execution.
///
/// When a Trap occurs, we process the frame info lazily for each
/// function in the frame. That way we minimize as much as we can
/// the upfront effort.
///
/// The data can also be processed upfront. This will happen in the case
/// of compiling at the same time that emiting the JIT.
/// In that case, we don't need to deserialize/process anything
/// as the data is already in memory.
#[derive(Clone)]
pub enum SerializableFunctionFrameInfo {
    /// The unprocessed frame info (binary)
    Unprocessed(UnprocessedFunctionFrameInfo),
    /// The processed frame info (memory struct)
    Processed(CompiledFunctionFrameInfo),
}

impl SerializableFunctionFrameInfo {
    /// Returns true if the extra function info is not yet
    /// processed
    pub fn is_unprocessed(&self) -> bool {
        matches!(self, Self::Unprocessed(_))
    }
}

// Below:
// The custom ser/de for `SerializableFunctionFrameInfo`.

impl Serialize for SerializableFunctionFrameInfo {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let unprocessed = match self {
            Self::Processed(processed) => UnprocessedFunctionFrameInfo::serialize(processed),
            Self::Unprocessed(unprocessed) => unprocessed.clone(),
        };
        s.serialize_bytes(&unprocessed.bytes)
    }
}

impl BorshSerialize for SerializableFunctionFrameInfo {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let unprocessed = match self {
            Self::Processed(processed) => UnprocessedFunctionFrameInfo::serialize(processed),
            Self::Unprocessed(unprocessed) => unprocessed.clone(),
        };
        BorshSerialize::serialize(&unprocessed.bytes, writer)
    }
}

struct FunctionFrameInfoVisitor;

impl<'de> Visitor<'de> for FunctionFrameInfoVisitor {
    type Value = UnprocessedFunctionFrameInfo;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("bytes")
    }
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        Ok(UnprocessedFunctionFrameInfo { bytes: v })
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(UnprocessedFunctionFrameInfo { bytes: v.to_vec() })
    }
}

impl<'de> Deserialize<'de> for SerializableFunctionFrameInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::Unprocessed(
            deserializer.deserialize_byte_buf(FunctionFrameInfoVisitor)?,
        ))
    }
}

impl BorshDeserialize for SerializableFunctionFrameInfo {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let u: UnprocessedFunctionFrameInfo = BorshDeserialize::deserialize(buf)?;
        for _ in 0..100 {
            let x = u.deserialize();
        }

        let r = Self::Unprocessed(u);
        Ok(r)
    }
}
