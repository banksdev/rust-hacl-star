extern crate hacl_star;

use hacl_star::box_;


const MSG: [u8; 104] = [
    // 32 bytes zero.
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
    0x16, 0x17, 0x18, 0x19, 0x20, 0x21, 0x22, 0x23,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
    0x16, 0x17, 0x18, 0x19, 0x20, 0x21, 0x22, 0x23,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
    0x16, 0x17, 0x18, 0x19, 0x20, 0x21, 0x22, 0x23,
];

const NONCE: [u8; 24] = [
    0x00, 0x01, 0x02, 0x03,
    0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x10, 0x11,
    0x12, 0x13, 0x14, 0x15,
    0x16, 0x17, 0x18, 0x19,
    0x20, 0x21, 0x22, 0x23,
];

const KEY: [u8; 32] = [
    0x85, 0xd6, 0xbe, 0x78,
    0x57, 0x55, 0x6d, 0x33,
    0x7f, 0x44, 0x52, 0xfe,
    0x42, 0xd5, 0x06, 0xa8,
    0x01, 0x03, 0x80, 0x8a,
    0xfb, 0x0d, 0xb2, 0xfd,
    0x4a, 0xbf, 0xf6, 0xaf,
    0x41, 0x49, 0xf5, 0x1b
];

const SK1: [u8; 32] = [
    0x85, 0xd6, 0xbe, 0x78,
    0x57, 0x55, 0x6d, 0x33,
    0x7f, 0x44, 0x52, 0xfe,
    0x42, 0xd5, 0x06, 0xa8,
    0x01, 0x03, 0x80, 0x8a,
    0xfb, 0x0d, 0xb2, 0xfd,
    0x4a, 0xbf, 0xf6, 0xaf,
    0x41, 0x49, 0xf5, 0x1b
];
const SK2: [u8; 32] = [
    0x85, 0xd6, 0xbe, 0x78,
    0x57, 0x55, 0x6d, 0x33,
    0x7f, 0x44, 0x52, 0xfe,
    0x42, 0xd5, 0x06, 0xa8,
    0x01, 0x03, 0x80, 0x8a,
    0xfb, 0x0d, 0xb2, 0xfd,
    0x4a, 0xbf, 0xf6, 0xaf,
    0x41, 0x49, 0xf5, 0x1c
];


#[test]
fn test_secretbox() {
    use box_::secret;

    let mut ct = [0; 72 + 32];
    let mut pt = [0; 72 + 32];
    let mut tag = [0; secret::MAC_LENGTH];

    secret::Key(&KEY).nonce(&NONCE)
        .seal(&MSG, &mut ct, &mut tag);

    let r = secret::Key(&KEY).nonce(&NONCE)
        .open(&mut pt, &ct, &tag);
    assert!(r);
    assert_eq!(&pt[..], &MSG[..]);
}

#[test]
fn test_sealedbox() {
    use hacl_star::curve25519;
    use box_::sealed;

    const BASEPOINT: [u8; 32] = [9; 32];

    let mut pk1 = [0; 32];
    let mut pk2 = [0; 32];
    let mut ct = [0; 72 + 32];
    let mut pt = [0; 72 + 32];
    let mut tag = [0; 16];

    curve25519::scalarmult(&mut pk1, &SK1, &BASEPOINT);
    curve25519::scalarmult(&mut pk2, &SK2, &BASEPOINT);

    let pk1 = sealed::PublicKey(pk1);
    let pk2 = sealed::PublicKey(pk2);

    sealed::SecretKey(SK1).and(&pk2).nonce(&NONCE)
        .seal(&MSG, &mut ct, &mut tag);

    let r = sealed::SecretKey(SK2).and(&pk1).nonce(&NONCE)
        .open(&mut pt, &ct, &tag);
    assert!(r);
    assert_eq!(&pt[..], &MSG[..]);
}
