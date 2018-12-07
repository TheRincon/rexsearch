use io::fasta;
use io::fastq;
use fastx_utils;

pub fn trunc_fasta(file_path: &str, threshold: usize) {
    let reader = fasta::Reader::from_file(file_path).unwrap();
    let mut writer = fasta::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for mut result in reader.records() {
        let mut rec = result.unwrap();
        if rec.seq_string().len() < threshold {
            writer.write_record(&mut rec);
        } else {
            let str_slice = &rec.seq_string()[..threshold];
            rec.update_seq(str_slice);
            writer.write_record(&mut rec).expect("Cannot write updated seq in trunc_fasta");
        }
    }
}

pub fn trunc_fastq(file_path: &str, threshold: usize) {
    let reader = fastq::Reader::from_file(file_path).unwrap();
    let mut writer = fastq::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for mut result in reader.records() {
        let mut rec = result.unwrap();
        if rec.seq_string().len() < threshold {
            writer.write_record(&mut rec);
        } else {
            let str_slice = &rec.seq_string()[..threshold];
            rec.update_seq(str_slice);
            writer.write_record(&mut rec).expect("Cannot write updated seq in trunc fastq");
        }
    }
}

pub fn trunc_last_fastq(file_path: &str, threshold: usize) {
    if threshold <= 0 {
        panic!("not a valid threshold");
    }
    let reader = fastq::Reader::from_file(file_path).unwrap();
    let mut writer = fastq::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for mut result in reader.records() {
        let mut rec = result.unwrap();
        if rec.seq_string().len() < threshold {
            writer.write_record(&mut rec);
        } else {
            let str_slice_first = &rec.seq_string().chars().rev().skip(threshold).collect::<String>();
            let str_slice = str_slice_first.chars().rev().collect::<String>();
            rec.update_seq(&str_slice);
            writer.write_record(&mut rec).expect("Cannot write updated seq in trunc_last_fastq");
        }
    }
}

pub fn trunc_last_fasta(file_path: &str, threshold: usize) {
    if threshold <= 0 {
        panic!("not a valid threshold");
    }
    let reader = fastq::Reader::from_file(file_path).unwrap();
    let mut writer = fastq::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for mut result in reader.records() {
        let mut rec = result.unwrap();
        if rec.seq_string().len() < threshold {
            writer.write_record(&mut rec);
        } else {
            let str_slice_first = &rec.seq_string().chars().rev().skip(threshold).collect::<String>();
            let str_slice = str_slice_first.chars().rev().collect::<String>();
            rec.update_seq(&str_slice);
            writer.write_record(&mut rec).expect("Cannot write updated seq in trunc_last_fastq");
        }
    }
}

/*  No idea what I am doing here. Just came back from a break
pub fn trunc_first_fasta(file_path: &str, threshold: usize, ending: &str) {
    if threshold <= 0 {
        panic!("not a valid threshold");
    }
    let mut ending = "a";
    if file_path.ends_with("fastq") || file_path.ends_with("fq") {
        ending = "fastq"
    } else if file_path.ends_with("fasta") || file_path.ends_with("fna") || file_path.ends_with("fa") || file_path.ends_with("fas") {
        ending = "fasta"
    } else {
        ending = "invalid"
    }
    let reader = fastq::Reader::from_file(file_path).unwrap();
    let mut writer = fastq::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for mut result in reader.records() {
        let mut rec = result.unwrap();
        if rec.seq_string().len() < threshold {
            writer.write_record(&mut rec);
        } else {
            let str_slice_first = &rec.seq_string().chars().rev().skip(threshold).collect::<String>();
            let str_slice = str_slice_first.chars().rev().collect::<String>();
            rec.update_seq(&str_slice);
            writer.write_record(&mut rec);
        }
    }
}
*/