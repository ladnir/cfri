#![allow(clippy::op_ref)]
pub mod arithmetic;
pub mod avx_int_types;
pub mod basefold;
pub mod binary_extension_fields;
pub mod binius_iter;
pub mod blaze;
pub mod blaze2;
pub mod blaze_transcript;
pub mod code;
pub mod expression;
pub mod ff_255;
pub mod goldilocksMont;
pub mod hash;
pub mod mersenne_61_mont;
pub mod new_fields;
pub mod parallel;
pub mod pcs;
pub mod piop;
pub mod play_field;
pub mod poly;
mod support;
mod timer;
pub mod transcript;

pub use halo2_curves;
pub use support::{
    chain, end_timer, izip, start_timer, start_unit_timer, BigUint, BitIndex, Deserialize,
    DeserializeOwned, Deserializer, Itertools, Serialize, Serializer,
};
pub(crate) use support::{impl_index, izip_eq};

#[cfg(feature = "benchmark")]
pub use support::test;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InvalidSumcheck(String),
    InvalidPcsParam(String),
    InvalidPcsOpen(String),
    InvalidSnark(String),
    Serialization(String),
    Transcript(std::io::ErrorKind, String),
}
