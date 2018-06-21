// use bio::io::fasta::Reader;
// use bio::io::fastq::Reader as Reader1;
use itertools::enumerate;
use io::fasta;
use io::fastq;

pub fn filter_fasta() {
    let reader = fasta::Reader::from_file("/home/danielw1234/Desktop/samp.fasta").unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().contains(&('N' as u8)) {
            rec.clear();
        }
    }
}

pub fn filter_fastq() {
    let reader = fastq::Reader::from_file("reads.fastq").unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().contains(&('N' as u8)) {
           rec.clear();
        }
    }
}