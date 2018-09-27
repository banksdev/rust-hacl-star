use hacl_star_sys as ffi;
use ::randombytes::randombytes;


const BASEPOINT: [u8; 32] = [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub const PUBLIC_LENGTH: usize = 32;
pub const SECRET_LENGTH: usize = 32;

define!{
    pub struct SecretKey/secretkey(pub [u8; SECRET_LENGTH]);
    pub struct PublicKey/publickey(pub [u8; SECRET_LENGTH]);
}

#[inline]
pub fn keypair(sk: &mut Option<SecretKey>, pk: &mut Option<PublicKey>) {
    let SecretKey(sk) = sk.get_or_insert(SecretKey([0; SECRET_LENGTH]));
    let PublicKey(pk) = pk.get_or_insert(PublicKey([0; PUBLIC_LENGTH]));

    randombytes(sk);
    scalarmult(pk, sk, &BASEPOINT);
}

impl SecretKey {
    #[inline]
    pub fn get_public(&self) -> PublicKey {
        let SecretKey(sk) = self;
        let mut pk = [0; 32];

        scalarmult(&mut pk, sk, &BASEPOINT);

        PublicKey(pk)
    }

    pub fn exchange(&self, &PublicKey(ref pk): &PublicKey, output: &mut [u8; 32]) {
        let SecretKey(sk) = self;

        scalarmult(output, sk, pk);
    }
}


pub fn scalarmult(mypublic: &mut [u8; 32], secret: &[u8; 32], basepoint: &[u8; 32]) {
    unsafe {
        ffi::curve25519::Hacl_Curve25519_crypto_scalarmult(
            mypublic.as_mut_ptr(),
            secret.as_ptr() as _,
            basepoint.as_ptr() as _
        );
    }
}
