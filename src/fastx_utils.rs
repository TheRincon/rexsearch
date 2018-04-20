use std::io;
use std::io::{BufWriter, BufReader};
use std::fs::File;
use std::io::BufRead;
use std::io::Write;

use dust;

pub fn read_fasta(file_path: String) {
    let mut new_file = file_path.to_owned();
    let f = File::open(file_path).expect("Could not read file, line 9: fastx_utils::read_fast(). Check file path");
    let file = BufReader::new(&f);
    let ending = new_file.find(".fasta");  // ".fasta" || ".fsa" || ".fna" || ".ffn" || ".frn" || "fa" || ".fas" || ".seq"
    let mut new_file_path = new_file[..ending.unwrap()].to_string();
    new_file_path.push_str("_new.fasta");
    read_write_fast_new(new_file_path, &f);
    // for (num, line) in file.lines().enumerate() {
    //    let mut l = line.unwrap();
    //    l.push_str("\n");
    // }
}

pub fn read_write_fast_new(name: String, filee: &File) {
    let f = File::create(name).expect("Could not create file, line 19: fastx_utils.rs::write_fast_new(). If Permission Denied error: File probably already exists.");
    let file = BufReader::new(filee);
    let mut writer = BufWriter::new(&f);
    for (num, line) in file.lines().enumerate() {
        let mut l = line.unwrap();
        let mut seq_vec: Vec<char> = l.chars().collect();
        if l.contains(">") {
            // dont apply dust
        } else {
            dust::dust(&mut seq_vec, true);
        }
        let mut seq_line = seq_vec.iter().cloned().collect::<String>();
        seq_line.push_str("\n");
        writer.write(&seq_line[..].as_bytes());
    }

}

pub fn write_fast_existing(file_path: String) {

}

pub fn read_fastq() {

}