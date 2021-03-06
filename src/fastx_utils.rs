
// crate imports
use io::fasta;
use bio::io::fastq;

// rust imports
use std::io::{BufWriter, BufReader};
use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::str;

// local imports
use dust;

pub fn read_file(file_path: String) -> BufReader<File> {
    let f = File::open(file_path).expect("Could not read file, line 9: fastx_utils::read_file(). Check file path");
    BufReader::new(f)
}

pub fn duster(file_path: String) {
    let reader = fasta::Reader::from_file(file_path).unwrap();
    let res = dust_fasta(reader);
}

pub fn dust_fasta(fasta: fasta::Reader<File>) -> fasta::Reader<File> {
    let reader = fasta::Reader::from_file("/home/danielw1234/Desktop/samp.fasta").unwrap();
    for record in fasta.records() {
        let mut result = record.unwrap();
        // dust::dust(&mut r, true);
    }
    reader
}

pub fn dust_fastq(fastq: fastq::Reader<File>) -> fastq::Reader<File> {
    let reader = fastq::Reader::from_file("/Users/daniel/Downloads/SP1.fq").expect("Could not open FASTQ for DUST");
    for record in fastq.records() {
        let result = record.unwrap();
        let mut seq = result.seq();
        // dust::dust(&mut seq, true);
    }
    reader
}


/*
fn parse_ffn(filee: BufReader<File>) {
    let mut rec_vec: Vec<char> = Vec::new();
    for line in filee.lines() {
        let mut l = line.unwrap();
        if l.starts_with(">") {

        } else {
            loop {
                let seq_line = l.trim();
                let mut seq_vec: Vec<char> = seq_line.chars().collect();
                rec_vec.append(&mut seq_vec);
            }
        }
    }
}
*/

// write regex to find   =>   ".fasta" || ".fsa" || ".fna" || ".ffn" || ".frn" || "fa" || ".fas" || ".seq"
// then find it and return index

/* pub fn extension_finder<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("(.fasta|.fsa|.frn|.ffn|.mpfa|.fna|.faa|.fa|.seq)$").unwrap();
        }
    let input = input.into();
    let mut matches
    if true {
        Cow::Owned(String::from("Test"))
    } else {
        input
    }
} */

pub fn create_new_fastx(file_path: String) -> File {
    let ending = file_path.find(".fasta");  // ".fasta" || ".fsa" || ".fna" || ".ffn" || ".frn" || "fa" || ".fas" || ".seq" || ".mpfa" || ".faa"
    let mut new_file_path = file_path[..ending.expect("Could not find file ending of .fasta")].to_string();
    new_file_path.push_str("_new.fasta");
    File::create(new_file_path).expect("Can't create new file here")
}

pub fn create_new_file_path(file_path: String) -> String {
    let ending = file_path.find(".fasta");  // ".fasta" || ".fsa" || ".fna" || ".ffn" || ".frn" || "fa" || ".fas" || ".seq" || ".mpfa" || ".faa"
    let mut new_file_path = file_path[..ending.expect("Could not find file ending of .fasta")].to_string();
    new_file_path.push_str("_new.fasta");
    new_file_path
}


// bufReader is faster than fasta::Reader by about 60 seconds
pub fn write_dust_fasta_new(name: String) {
    let f_path = name.to_owned();
    let f = create_new_fastx(name);
    let filee = read_file(f_path);
    let mut writer = BufWriter::new(f);
    for line in filee.lines() {
        let mut l = line.unwrap();
        if l.find(">") >= Some(0) {
            writer.write(&l.as_bytes());
            writer.write(&[10u8]);
        } else {
            dust::dust(&mut l, true);
            writer.write(&l.as_bytes());
            writer.write(&[10u8]);
        }
    }
}

pub fn write_fast_existing(file_path: String) {
    let _ = read_file(file_path);
}

pub fn read_fastq() {

}

