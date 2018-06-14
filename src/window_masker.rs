pub fn window_masker(k: &[String]) {
    let nmer_len = nmer_estimate(get_len_contigs(k));
    let
    for i in 0..k.len() {
        nmer_scanner(k[i], nmer_len);
    }
}

pub fn nmer_estimate(estimate_sum: i64) -> i64 {
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

#[inline]
pub fn get_len_contigs(contigs: &[String]) -> i64 {
    let mut j = 0;
    for i in 0..contigs.len() {
        j += contigs[i].len();
    }
    println!("{:?}", j);
    return j as i64;
}

pub fn nmer_scanner(contig: &str, nmer_len: i64) -> &str {
    if nmer_len > contig.len() as i64 {
        return contig;
    }
    let nmer = "";
    let z: &str = "dsd";
    z
}