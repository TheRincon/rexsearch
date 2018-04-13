use std::num;

pub fn window_masker(k: &[i64]) {
    let y = nmer_estimate(&k);
}

pub fn nmer_estimate(lengths: &[i64]) -> i64 {
    let estimate_sum: i64 = lengths.iter().sum();
    for i in 1..1_000 {
        if estimate_sum / ( 4_i64.pow(i as u32)) < 5 {
            return i as i64;
        }
        if i == 1000 {
            panic!("No nmer found for contigs length");
        }
    }
    return 0;
}