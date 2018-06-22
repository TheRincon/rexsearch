


// calculate number of kmers here  N = L-K+1
pub fn kmer_number_per_strand(nmer_len: i64, strand_len: i64) -> i64 {
    if nmer_len > strand_len {
        return 0;
    }
    return strand_len - nmer_len + 1;
}

// number of possible kmers 4^K
#[inline]
pub fn possible_kmers(nmer_len: i64) -> i64 {
    return 4_i64.pow(nmer_len as u32);
}

