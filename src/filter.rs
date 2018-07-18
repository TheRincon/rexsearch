// use bio::io::fasta::Reader;
// use bio::io::fastq::Reader as Reader1;
use itertools::enumerate;
use io::fasta;
use io::fastq;
use fastx_utils;

pub fn filter_fasta_n(file_path: &str) {
    let reader = fasta::Reader::from_file(file_path).unwrap();
    let mut writer = fasta::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().contains(&('N' as u8)) {
            continue
        } else {
            writer.write_record(&mut rec);
        }
    }
}

pub fn filter_fastq_n(file_path: &str) {
    let reader = fastq::Reader::from_file(file_path).unwrap();
    let mut writer = fastq::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().contains(&('N' as u8)) {
            continue
        } else {
            writer.write_record(&mut rec);
        }
    }
}

// v.iter().filter(|&n| *n == 91).count()
pub fn filter_fastq_count_n(file_path: &str, num: usize) {
    let reader = fastq::Reader::from_file(file_path).unwrap();
    let mut writer = fastq::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().iter().filter(|&n| *n == 'N' as u8).count() > num {
            continue
        } else {
            writer.write_record(&mut rec);
        }
    }
}

pub fn filter_fasta_count_n(file_path: &str, num: usize) {
    let reader = fasta::Reader::from_file(file_path).unwrap();
    let mut writer = fasta::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().iter().filter(|&n| *n == 'N' as u8).count() > num {
            continue
        } else {
            writer.write_record(&mut rec);
        }
    }
}

pub fn filter_fasta_max_len(file_path: &str, len: usize) {
    let reader = fasta::Reader::from_file(file_path).unwrap();
    let mut writer = fasta::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().len() > len {
            continue
        } else {
            writer.write_record(&mut rec);
        }
    }
}

pub fn filter_fastq_min_len(file_path: &str, len: usize) {
    let reader = fastq::Reader::from_file(file_path).unwrap();
    let mut writer = fastq::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.seq().len() < len {
            continue
        } else {
            writer.write_record(&mut rec);
        }
    }
}

pub fn filter_fastq_max_quality(file_path: &str, qual: i64) {
    let reader = fastq::Reader::from_file(file_path).unwrap();
    let mut writer = fastq::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.qual().iter().filter(|&&x| x > qual as u8 ).count() > 0 {
            continue
        } else {
            writer.write_record(&mut rec);
        }
    }
}

pub fn filter_fastq_min_quality(file_path: &str, qual: char) {
    let reader = fastq::Reader::from_file(file_path).unwrap();
    let thresh = format!("");
    let mut writer = fastq::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();
        if rec.qual().iter().filter(|&&x| x > qual as u8 ).count() > 0 {
                continue
        } else {
            writer.write_record(&mut rec);
        }
    }
}