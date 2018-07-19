/*
    Window masker in rust
    https://academic.oup.com/bioinformatics/article/22/2/134/424703
    Useful for masking as an alternative to DUST. RepeatModeler and
    RepeatMasker are too involved for this task.
*/

use std::collections::HashMap;
use std::str;
use itertools::enumerate;
use dna_utils;

pub fn window_masker(k: &[String]) {
    let mut rev_vec = Vec::new();
    for m in k {
        let mut j = m.clone();
        let r = dna_utils::rev_comp(&mut j);
        rev_vec.push(r);
    }
    wm(&k, &rev_vec)
}

pub fn wm(k: &[String], rev_vec: &[String]) {
    let nmer_len = nmer_estimate(get_len_contigs(k));
    let mut kmer_map = HashMap::new();
    for (i,s) in enumerate(k) {
            nmer_scanner(s, &rev_vec[i], nmer_len, &mut kmer_map);
    }
    let (std_dev, mean, size) = get_hashmap_stats(&kmer_map);
    let (t_high, t_thresh, t_extend, t_low) = get_thresholds(std_dev);
}

// 100 billion is around 18 so it better be huge to get 40 --> L/(4^K) < 5
pub fn nmer_estimate(estimate_sum: i64) -> i64 {
    for i in 1..40 {
        if estimate_sum / ( 4_i64.pow(i as u32)) < 5 {
            if i < 2 { return 2 }
            return i as i64;
        }
        if i == 40 {
            panic!("No nmer found for contigs length");
        }
    }
    return 0;
}

#[inline]
pub fn get_len_contigs(contigs: &[String]) -> i64 {
    contigs.iter().fold(0, |a: i64, ref b| a + b.len() as i64 )
}

pub fn nmer_scanner<'a>(contig: &'a str, rev_contig: &'a str, nmer_len: i64, hash_map: &mut HashMap<&'a str, i64>) {
    if contig.len() < nmer_len as usize {
        return;
    }
    for i in 0..contig.len() - nmer_len as usize + 1 {
        // ugly
        hash_map.entry(& contig[i..(i + nmer_len as usize)]).and_modify(|e| { *e += 1 }).or_insert(1);
        hash_map.entry(& rev_contig[i..(i + nmer_len as usize)]).and_modify(|x| { *x += 1 }).or_insert(1);
    }
}

pub fn get_hashmap_stats(hashmap: &HashMap<&str, i64>) -> (f64, i64, i64) {
    let size = hashmap.keys().len() as i64;
    let sum: i64 = hashmap.values().into_iter().sum();
    let u = sum / size;
    let stdd: i64 = hashmap.values().into_iter().map(|&x| (x - u) * (x - u)).sum();
    let std_dev = ((stdd / size - 1) as f64).sqrt();     // Bessel's correction of the kmer count
    return (std_dev, u, size)
}

pub fn get_thresholds(std_dev: f64) -> (i64, i64, i64, i64) {
    // 3 sigma is around 99.9%
    // 2.575829 sigma is 99.5%
    // 2.33 sigma is ~99.0%
    // 1.644854 sigma is 90.0%
    // CDF here, need 99.8, 99.5, 99.0, 90.0 for T_High, T_Thresh, T_Extend, and T_Low respectively.
    let sig_998 = (std_dev * 2.87816).round() as i64; // T_High
    let sig_995 = (std_dev * 2.57583).round() as i64; // T_Thresh
    let sig_990 = (std_dev * 2.32635).round() as i64; // T_Extend
    let sig_900 = (std_dev * 1.64485).round() as i64; // T_Low
    return (sig_998, sig_995, sig_990, sig_900)
}

// Even when the genome is 100 Billion the "K" should never exceed ~18 --> L/(4^K) < 5
// Probably don't need this because I will not generate in advance, too much memory consumed.
// I will just count them as I go with HashMap.

pub fn kmer_generator(depth: i64, base: String, kmers: &mut [String], mut kmers_offset: i64, kmer_length: i64) {
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