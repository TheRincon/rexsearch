use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct Args {
    fastas: Vec<PathBuf>,
    fastqs: Vec<PathBuf>,
    identity_threshold: f64,
    output_file: File,

}

impl Args {

}