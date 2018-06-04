use std::io::{BufWriter, BufReader};
use std::fs::File;
use std::io::BufRead;
use std::io::Write;

use dust;

pub fn read_file(file_path: String) -> BufReader<File> {
    let f = File::open(file_path).expect("Could not read file, line 9: fastx_utils::read_file(). Check file path");
    BufReader::new(f)
}

fn parse_fasta(filee: BufReader<File>) {

    for line in filee.lines() {
        let mut l = line.unwrap();
        if l.starts_with(">") {
            let mut rec_vec: Vec<char> = Vec::new();

        } else {
            loop {
                let seq_line = l.trim();
                let mut seq_vec: Vec<char> = seq_line.chars().collect();
                rec_vec.append(&mut seq_vec);
            }
        }
    }
}

// write regex to find   =>   ".fasta" || ".fsa" || ".fna" || ".ffn" || ".frn" || "fa" || ".fas" || ".seq"
// then find it and return index

pub fn create_new_fastx(file_path: String) -> File {

    let ending = file_path.find(".fasta");  // ".fasta" || ".fsa" || ".fna" || ".ffn" || ".frn" || "fa" || ".fas" || ".seq"
    let mut new_file_path = file_path[..ending.unwrap()].to_string();
    new_file_path.push_str("_new.fasta");
    File::create(new_file_path).expect("Can't create new file here")
}



fn dust_collecter(dust_pile: &mut [char]) {

}

pub fn write_fasta_new(name: String) {
    let f_path = name.to_owned();
    let f = create_new_fastx(name);
    // let f = File::create(name).expect("Could not create file, line 19: fastx_utils.rs::write_fast_new(). If Permission Denied error: File probably already exists.");
    let filee: BufReader<File> = read_file(f_path);
    let mut writer = BufWriter::new(f);
    for line in filee.lines() {
        let mut l = line.unwrap();
        let mut seq_vec: Vec<char> = l.chars().collect();
        if !l.contains(">") {
            dust::dust(&mut seq_vec, true);
        }
        let mut seq_line = seq_vec.iter().cloned().collect::<String>();
        seq_line.push_str("\n");
        writer.write(&seq_line[..].as_bytes());
    }

}

pub fn write_fast_existing(file_path: String) {
    let _ = read_file(file_path);
}

pub fn read_fastq() {

}

