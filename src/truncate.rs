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
            writer.write_record(&mut rec);
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
            writer.write_record(&mut rec);
        }
    }
}