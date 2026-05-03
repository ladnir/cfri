use crate::backend::{
    arithmetic::{fe_mod_from_le_bytes, PrimeField},
    binary_extension_fields::B128,
    hash::{Blake2s, Hash, Output},
    new_fields::{Mersenne127, Mersenne61},
    transcript as legacy_transcript, Error,
};
use generic_array::{ArrayLength, GenericArray};
use halo2_curves::{bn256, pasta};
use std::io::{self, Cursor, Read, Write};

const DOMAIN: &[u8] = b"cfri-transcript-v1";
const ABSORB_BYTES: &[u8] = b"absorb-bytes";
const ABSORB_FIELD: &[u8] = b"absorb-field";
const SQUEEZE: &[u8] = b"squeeze";

pub trait Absorb {
    fn absorb_into<H: Hash>(&self, state: &mut H);
}

pub trait Squeeze: Sized {
    fn squeeze_from<H: Hash>(state: &mut H) -> Self;
}

#[derive(Clone, Debug)]
pub struct Transcript<H: Hash = Blake2s, S = Cursor<Vec<u8>>> {
    state: H,
    stream: S,
}

pub type Blake2sTranscript<S = Cursor<Vec<u8>>> = Transcript<Blake2s, S>;

impl<H: Hash> Default for Transcript<H, Cursor<Vec<u8>>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H: Hash> Transcript<H, Cursor<Vec<u8>>> {
    pub fn new() -> Self {
        Self::with_stream(Cursor::new(Vec::new()))
    }

    pub fn into_proof(self) -> Vec<u8> {
        self.stream.into_inner()
    }

    pub fn from_proof(proof: &[u8]) -> Self {
        Self::with_stream(Cursor::new(proof.to_vec()))
    }
}

impl<H: Hash, S> Transcript<H, S> {
    pub fn with_stream(stream: S) -> Self {
        let mut state = H::new();
        state.update(DOMAIN);
        Self { state, stream }
    }

    pub fn absorb<T: Absorb + ?Sized>(&mut self, value: &T) {
        value.absorb_into(&mut self.state);
    }

    pub fn absorb_slice<T: Absorb>(&mut self, values: &[T]) {
        absorb_len::<H>(&mut self.state, values.len());
        for value in values {
            self.absorb(value);
        }
    }

    pub fn squeeze<T: Squeeze>(&mut self) -> T {
        T::squeeze_from(&mut self.state)
    }

    pub fn squeeze_into<T: Squeeze>(&mut self, out: &mut [T]) {
        for value in out {
            *value = self.squeeze();
        }
    }
}

impl<H: Hash> legacy_transcript::InMemoryTranscript for Transcript<H, Cursor<Vec<u8>>> {
    type Param = ();

    fn new(_: Self::Param) -> Self {
        Self::new()
    }

    fn into_proof(self) -> Vec<u8> {
        self.into_proof()
    }

    fn from_proof(_: Self::Param, proof: &[u8]) -> Self {
        Self::from_proof(proof)
    }
}

impl<H, F, S> legacy_transcript::FieldTranscript<F> for Transcript<H, S>
where
    H: Hash,
    F: Absorb + Squeeze,
{
    fn squeeze_challenge(&mut self) -> F {
        self.squeeze()
    }

    fn common_field_element(&mut self, fe: &F) -> Result<(), Error> {
        self.absorb(fe);
        Ok(())
    }
}

impl<H, F, R> legacy_transcript::FieldTranscriptRead<F> for Transcript<H, R>
where
    H: Hash,
    F: Absorb + Squeeze + PrimeField,
    R: Read,
{
    fn read_field_element(&mut self) -> Result<F, Error> {
        let mut repr = <F as PrimeField>::Repr::default();
        self.stream
            .read_exact(repr.as_mut())
            .map_err(|err| Error::Transcript(err.kind(), err.to_string()))?;
        repr.as_mut().reverse();
        let fe = F::from_repr_vartime(repr).ok_or_else(|| {
            Error::Transcript(
                io::ErrorKind::Other,
                "Invalid field element encoding in proof".to_string(),
            )
        })?;
        self.absorb(&fe);
        Ok(fe)
    }
}

impl<H, F, W> legacy_transcript::FieldTranscriptWrite<F> for Transcript<H, W>
where
    H: Hash,
    F: Absorb + Squeeze + PrimeField,
    W: Write,
{
    fn write_field_element(&mut self, fe: &F) -> Result<(), Error> {
        self.absorb(fe);
        let mut repr = fe.to_repr();
        repr.as_mut().reverse();
        self.stream
            .write_all(repr.as_ref())
            .map_err(|err| Error::Transcript(err.kind(), err.to_string()))
    }
}

impl<H, C, F, S> legacy_transcript::Transcript<C, F> for Transcript<H, S>
where
    H: Hash,
    C: Absorb,
    F: Absorb + Squeeze,
{
    fn common_commitment(&mut self, comm: &C) -> Result<(), Error> {
        self.absorb(comm);
        Ok(())
    }
}

impl<H, N, F, R> legacy_transcript::TranscriptRead<GenericArray<u8, N>, F> for Transcript<H, R>
where
    H: Hash,
    N: ArrayLength<u8>,
    F: Absorb + Squeeze + PrimeField,
    R: Read,
{
    fn read_commitment(&mut self) -> Result<GenericArray<u8, N>, Error> {
        let mut hash = GenericArray::<u8, N>::default();
        self.stream
            .read_exact(hash.as_mut())
            .map_err(|err| Error::Transcript(err.kind(), err.to_string()))?;
        self.absorb(&hash);
        Ok(hash)
    }
}

impl<H, N, F, W> legacy_transcript::TranscriptWrite<GenericArray<u8, N>, F> for Transcript<H, W>
where
    H: Hash,
    N: ArrayLength<u8>,
    F: Absorb + Squeeze + PrimeField,
    W: Write,
{
    fn write_commitment(&mut self, hash: &GenericArray<u8, N>) -> Result<(), Error> {
        self.absorb(hash);
        self.stream
            .write_all(hash.as_ref())
            .map_err(|err| Error::Transcript(err.kind(), err.to_string()))
    }
}

impl Absorb for [u8] {
    fn absorb_into<H: Hash>(&self, state: &mut H) {
        state.update(ABSORB_BYTES);
        absorb_len::<H>(state, self.len());
        state.update(self);
    }
}

impl Absorb for str {
    fn absorb_into<H: Hash>(&self, state: &mut H) {
        self.as_bytes().absorb_into(state);
    }
}

impl<const N: usize> Absorb for [u8; N] {
    fn absorb_into<H: Hash>(&self, state: &mut H) {
        self.as_slice().absorb_into(state);
    }
}

impl<N: ArrayLength<u8>> Absorb for GenericArray<u8, N> {
    fn absorb_into<H: Hash>(&self, state: &mut H) {
        self.as_slice().absorb_into(state);
    }
}

fn absorb_len<H: Hash>(state: &mut H, len: usize) {
    state.update(&(len as u64).to_le_bytes());
}

fn squeeze_hash<H: Hash>(state: &mut H) -> Output<H> {
    state.update(SQUEEZE);
    let hash = state.finalize_fixed_reset();
    state.update(&hash);
    hash
}

macro_rules! impl_absorb_field {
    ($($field:ty),* $(,)?) => {
        $(
            impl Absorb for $field {
                fn absorb_into<H: Hash>(&self, state: &mut H) {
                    state.update(ABSORB_FIELD);
                    state.update(self.to_repr().as_ref());
                }
            }
        )*
    };
}

macro_rules! impl_squeeze_prime_field {
    ($($field:ty),* $(,)?) => {
        $(
            impl Squeeze for $field {
                fn squeeze_from<H: Hash>(state: &mut H) -> Self {
                    fe_mod_from_le_bytes(squeeze_hash(state))
                }
            }
        )*
    };
}

impl_absorb_field!(
    B128,
    Mersenne61,
    Mersenne127,
    bn256::Fr,
    pasta::Fp,
    pasta::Fq,
);

impl_squeeze_prime_field!(Mersenne61, Mersenne127, bn256::Fr, pasta::Fp, pasta::Fq,);

impl Squeeze for B128 {
    fn squeeze_from<H: Hash>(state: &mut H) -> Self {
        let hash = squeeze_hash(state);
        let mut repr = [0u8; 16];
        repr.copy_from_slice(&hash[..16]);
        B128::from_repr(repr).unwrap()
    }
}
