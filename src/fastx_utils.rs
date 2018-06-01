use std::io::{BufWriter, BufReader};
use std::fs::File;
use std::io::BufRead;
use std::io::Write;

use dust;

pub fn read_fasta(file_path: String) -> BufReader<&'static File> {
    let f = File::open(file_path).expect("Could not read file, line 9: fastx_utils::read_fast(). Check file path");
    BufReader::new(&f)
}

pub fn create_new_fastx(file_path: String) -> File {

    let ending = file_path.find(".fasta");  // ".fasta" || ".fsa" || ".fna" || ".ffn" || ".frn" || "fa" || ".fas" || ".seq"
    let mut new_file_path = file_path[..ending.unwrap()].to_string();
    new_file_path.push_str("_new.fasta");
    File::create(new_file_path).expect("Can't create new file here")
}

pub fn write_fasta_new(name: String) {
    let f_path = name.to_owned();
    let f = create_new_fastx(name);
    // let f = File::create(name).expect("Could not create file, line 19: fastx_utils.rs::write_fast_new(). If Permission Denied error: File probably already exists.");
    let filee: BufReader<&File> = read_fasta(f_path);
    let mut writer = BufWriter::new(&f);
    for (_, line) in filee.lines().enumerate() {
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
    let _ = read_fasta(file_path);
}

pub fn read_fastq() {

}

