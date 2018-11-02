use std::convert::AsRef;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct RNAseq {
    transcript: String,
    seq: String,
    introns: introns,
    exons: exon,
}

#[derive(Debug, Clone, Default)]
pub struct Transcript {
    transcript_part: String,
    seq: String,
    indices: (String, i64, i64),
    bundle: Vec<>
}

#[derive(Debug, Clone, Default)]
pub struct Exon {
    transcript_part: String,
    seq: String,
    indices: (i64, i64),
}

#[derive(Debug, Clone, Default)]
pub struct Intron {
    transcript_part: String,
    seq: String,
    indices: (i64, i64),
}

impl RNAseq {
    /// Create a new, empty FastQ record.
    pub fn new() -> Self {
        RNAseq {
            transcript: String::new(),
            seq: String::new(),
            introns: Vec::with_capacity(100),
            exons: Vec::with_capacity(100),
        }
    }

    pub fn with_seq(self, s: String) -> Self {
        self.seq = s;
        self
    }

    pub fn with_introns(self, ins: &mut [i64]) -> Self {
        self.introns = ins;
        self
    }

    pub fn with_exons(self, exs: &mut [i64]) -> Self {
        self.exons = exs;
        self
    }

    pub fn with_transcript(self, ts: String) -> Self {
        self.transcript = ts;
        self
    }
}


/// AFT Record
#[derive(Debug, Clone, Default)]
pub struct Record {
    gen: String,
    seq: String,
    rna: String,
}

impl Record {
    /// Create a new, empty FastQ record.
    pub fn new() -> Self {
        Record {
            gen: String::new(),
            seq: String::new(),
            rna: RNAseq,
        }
    }
}