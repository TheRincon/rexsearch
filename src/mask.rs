use io::fasta;
use fastx_utils;
use std::io::{BufWriter, BufReader};
use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::str;

pub fn soft_to_hard_mask(file_path: &str) {
    let reader = fasta::Reader::from_file(file_path.to_string()).unwrap();
    let mut writer = fasta::Writer::to_file(fastx_utils::create_new_file_path(file_path.to_string())).unwrap();
    for mut result in reader.records() {
        let mut rec = result.unwrap();
        let mut g = rec.seq_string();
        let y = mask_helper(&mut g);
        rec.update_seq(&y);
        writer.write_record(&mut rec);
    }
}

fn mask_helper(nts: &str) -> String {
    let mut x: Vec<char> = nts.chars().collect();
    let mut y = String::with_capacity(x.len());
    for mut i in x {
        match i {
            'a' | 'c' | 'g' | 't' => y.push('N'),
            _ => y.push(i),
        }
    }
    y
}

pub fn fast_mask(name: &str) {
    let f_path = name.to_owned();
    let f = fastx_utils::create_new_fastx(name.to_string());
    let filee = fastx_utils::read_file(f_path);
    let mut writer = BufWriter::new(f);
    for line in filee.lines() {
        let mut l = line.unwrap();
        if l.find(">") >= Some(0) {
            writer.write(&l.as_bytes()).expect("Cannot write fast_mask header as bytes");
            writer.write(&[10u8]).expect("Cannot write fast_mask newline as byte");
        } else {
            let x = mask_helper(&mut l);
            writer.write(&x.as_bytes()).expect("Cannot write fast_mask line as bytes");
            writer.write(&[10u8]).expect("Cannot write fast_mask newline as byte");
        }
    }
}
