// use bio::io::fasta::Reader;
// use bio::io::fastq::Reader as Reader1;
use itertools::enumerate;
use io::fasta;
use io::fastq;
use fastx_utils;

pub fn filter_fasta(file_path: &str) {
    let reader = fasta::Reader::from_file(file_path).unwrap();
    let mut writer = fasta::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().contains(&('N' as u8)) || rec.seq().contains(&('n' as u8)) {
            continue
        } else {
            writer.write_record(&rec);
        }
    }
}

pub fn filter_fastq(file_path: &str) {
    let reader = fastq::Reader::from_file(file_path).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().contains(&('N' as u8)) || rec.seq().contains(&('n' as u8)) {
           rec.clear();
        }
    }
}