pub mod binary_rs;
mod raa;
pub use raa::{
    encode_bits, encode_bits_long, encode_bits_ser, repetition_code_long, serial_accumulator_long,
    PackedRaaCode, Permutation, RaaSymbol,
};

pub trait LinearCodes<F>: Sync + Send {
    fn row_len(&self) -> usize;

    fn codeword_len(&self) -> usize;

    fn num_column_opening(&self) -> usize;

    fn num_proximity_testing(&self) -> usize;

    fn encode(&self, input: impl AsMut<[F]>);
}
