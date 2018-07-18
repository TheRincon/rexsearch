



pub fn collect_strings(file_path: &str) -> Vec<String> {
    let reader = fasta::Reader::from_file(file_path.to_string()).unwrap();
    let mut string_vec = Vec::new();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();

        string_vec.push(rec.seq_string())
    }
    string_vec
}