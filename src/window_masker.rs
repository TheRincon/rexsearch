// pub fn window_masker(k: &[i64]) {
//   let y = nmer_estimate(&k);
//}

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

/*pub fn get_len_contigs(contigs: &[str]) {
    let mut j = 0;
    for i in 0..contigs.len() {
        j += contigs[i].len();
    }
    return j;
}

pub fn nmer_scanner(contig: &str, nmer_len: i64) -> &str {
    if nmer_len > contig.len() {
        return contig;
    }
    // let nmer = []

} */