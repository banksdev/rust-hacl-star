/* automatically generated by rust-bindgen */

pub type __uint8_t = ::libc::c_uchar;
pub type __uint32_t = ::libc::c_uint;
pub type Hacl_Chacha20_uint8_p = *mut u8;
pub type Hacl_Chacha20_uint32_t = u32;
extern "C" {
    pub fn Hacl_Chacha20_chacha20_key_block(block: *mut u8, k: *mut u8, n1: *mut u8, ctr: u32);
}
extern "C" {
    pub fn Hacl_Chacha20_chacha20(
        output: *mut u8,
        plain: *mut u8,
        len: u32,
        k: *mut u8,
        n1: *mut u8,
        ctr: u32,
    );
}