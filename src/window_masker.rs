use std::collections::HashMap;

pub fn window_masker(k: &[String]) {
    let nmer_len = nmer_estimate(get_len_contigs(k));
    let mut placeholder = String::from("");
    for i in 0..nmer_len {
        placeholder.push_str("a");
    }
    for i in 0..k.len() {
        nmer_scanner(&k[i], nmer_len);
    }
}

// 100 billion is around 18 so it better be huge to get 50 --> L/(4^K) < 5
// Taken from WindowMasker Docs:
//  https://academic.oup.com/bioinformatics/article/22/2/134/424703

pub fn nmer_estimate(estimate_sum: i64) -> i64 {
    for i in 1..1_000 {
        if estimate_sum / ( 4_i64.pow(i as u32)) < 5 {
            if i < 2 { return 2 }
            return i as i64;
        }
        if i == 50 {
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
    return j as i64;
}

pub fn nmer_scanner(contig: &str, nmer_len: i64) -> Vec<&str> {

    if nmer_len > contig.len() as i64 {
        return vec![contig];
    }
    // let mut nmers: Vec<&str> = vec!["fives"; contig.len()];
    let mut kmer_map = HashMap::new();
    for i in 0..contig.len() - nmer_len as usize + 1 {
        let kmer_count = kmer_map.entry(&contig[i..(i + nmer_len as usize)]).or_insert(0);
        *kmer_count += 1;
    }
    vec!["sgd"]
}


// Even when the genome is 100 Billion the "K" should never exceed ~18 --> L/(4^K) < 5
// Probably don't need this because I will not generate in advance, too much mmory consumed.
// I will just count them as I go.

/* pub fn kmer_generator(depth: i64, base: String, kmers: &mut [String], mut kmers_offset: i64, kmer_length: i64) {

    if(depth == kmer_length) {
        &kmers[kmers_offset as usize]; // .assign(base) ???
        kmers_offset += 1;
    } else {
        let bases = ["A", "C", "T", "G"];
        for i in 0..4 {
            kmer_generator(depth + 1, base.clone() + bases[i], kmers, kmers_offset, 3);
        }
    }
}
*/