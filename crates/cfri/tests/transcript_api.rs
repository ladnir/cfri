use cfri::{backend::binary_extension_fields::B128, transcript::Blake2sTranscript};
use halo2_curves::{bn256::Fr, ff::Field};

#[test]
fn transcript_is_deterministic_across_field_types() {
    let mut a = Blake2sTranscript::new();
    a.absorb("domain");
    a.absorb(&Fr::from(7));
    a.absorb_slice(&[B128::from(11), B128::from(19)]);

    let mut b = Blake2sTranscript::new();
    b.absorb("domain");
    b.absorb(&Fr::from(7));
    b.absorb_slice(&[B128::from(11), B128::from(19)]);

    assert_eq!(a.squeeze::<Fr>(), b.squeeze::<Fr>());
    assert_eq!(a.squeeze::<B128>(), b.squeeze::<B128>());
}

#[test]
fn transcript_absorb_order_matters() {
    let mut left = Blake2sTranscript::new();
    left.absorb(&Fr::from(1));
    left.absorb(&Fr::from(2));

    let mut right = Blake2sTranscript::new();
    right.absorb(&Fr::from(2));
    right.absorb(&Fr::from(1));

    assert_ne!(left.squeeze::<Fr>(), right.squeeze::<Fr>());
}

#[test]
fn transcript_squeeze_into_uses_caller_buffer() {
    let mut bulk = Blake2sTranscript::new();
    bulk.absorb(b"seed");
    let mut out = [Fr::ZERO; 4];
    bulk.squeeze_into(&mut out);

    let mut scalar = Blake2sTranscript::new();
    scalar.absorb(b"seed");
    let expected = [
        scalar.squeeze::<Fr>(),
        scalar.squeeze::<Fr>(),
        scalar.squeeze::<Fr>(),
        scalar.squeeze::<Fr>(),
    ];

    assert_eq!(out, expected);
}

#[test]
fn transcript_distinguishes_concat_from_sliced_absorb() {
    let mut concat = Blake2sTranscript::new();
    concat.absorb(b"abcd");

    let mut split = Blake2sTranscript::new();
    split.absorb(b"ab");
    split.absorb(b"cd");

    assert_ne!(concat.squeeze::<B128>(), split.squeeze::<B128>());
}
