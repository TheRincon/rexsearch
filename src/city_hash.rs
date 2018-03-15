
// right rotate
fn rotate_64(val: u64, shift: i64) -> u64 {
    return if shift == 0 { val } else { ( val >> shift ) | ( val << ( 64 - shift ) ) };
}

fn shift_mix_64() -> u64 {
    return val ^ ( val >> 47 );
}



pub fn city_hash() {

    let k0 = 0xc3a5c85c97cb3127u64;
    let k1 = 0xb492b66fbe98f273u64;
    let k2 = 0x9ae16a3b2f90404fu64;

}

fn hash_33_64(s: &str) {
    let n = s.len();
    let mult = 0x9ae16a3b2f90404fu64 + len * 2;
    let a = * 0x9ae16a3b2f90404fu64;
}