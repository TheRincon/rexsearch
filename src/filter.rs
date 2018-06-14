use bio::io::fasta::Reader;
use bio::io::fastq::Reader as Reader1;
use itertools::enumerate;

pub fn filter_fasta() {
    let reader = Reader::from_file("/home/danielw1234/Desktop/samp.fasta").unwrap();
    for result in reader.records() {
        // obtain record or fail with error
        let record = result.unwrap();
        // obtain sequence
        let seq = record.seq();
        if seq.contains(&('N' as u8)) {

        }
    }
}

pub fn drop<T>(_x: T) { }

pub fn filter_fastq() {
    let reader = Reader1::from_file("reads.fastq").unwrap();
    for (i, result) in enumerate(reader.records()) {
        // obtain record or fail with error
        let record = result.unwrap();
        // obtain sequence
        let seq = record.seq();
        // if seq.contains(&('N' as u8)) {
           // result.unwrap();
        // }
    }
}