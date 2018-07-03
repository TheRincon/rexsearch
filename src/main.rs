/*
  VSEARCH: a versatile open source tool for metagenomics
  Copyright (C) 2014-2018, Torbjorn Rognes, Frederic Mahe and Tomas Flouri
  All rights reserved.

  Contact: Torbjorn Rognes <torognes@ifi.uio.no>,
  Department of Informatics, University of Oslo,
  PO Box 1080 Blindern, NO-0316 Oslo, Norway

  This software is dual-licensed and available under a choice
  of one of two licenses, either under the terms of the GNU
  General Public License version 3 or the BSD 2-Clause License.
  GNU General Public License version 3

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
  The BSD 2-Clause License

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

  1. Redistributions of source code must retain the above copyright
  notice, this list of conditions and the following disclaimer.

  2. Redistributions in binary form must reproduce the above copyright
  notice, this list of conditions and the following disclaimer in the
  documentation and/or other materials provided with the distribution.

  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
  "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
  LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
  FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
  COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
  BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
  LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
  LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
  ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
  POSSIBILITY OF SUCH DAMAGE.
*/

extern crate rand;
extern crate bio;
#[macro_use] extern crate clap;
extern crate itertools;
extern crate csv;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate statrs;

pub mod abundance;
pub mod dust;
pub mod window_masker;
pub mod shuffle;
pub mod fastx_utils;
pub mod io;
pub mod filter;
pub mod dna_utils;

use std::env;

use clap::{App, Arg, SubCommand};
use io::fasta;
use itertools::enumerate;

fn main() {

    let args: Vec<String> = env::args().collect();
    // let yaml = load_yaml!("cli.yml");
    // let matches = App::from_yaml(yaml).get_matches();
    // fastx_utils::write_fasta_new("/Users/daniel/Downloads/samp.fasta".to_string());
    // filter::filter_fasta_n("/Users/daniel/Desktop/samp.fasta");
    // fastx_utils::duster("/home/danielw1234/Desktop/samp.fasta".to_string());
    // let k = collect_strings(&args[1]);
    //window_masker::window_masker(&k[..]);
    //dust::dust_seqs(&args[1], &args[2], &args[3]);
    filter::filter_fasta_n(&args[1]);

    // let mut p = String::from("ATTAAAG");
    // let c = window_masker::rev_comp(&mut p);
    // println!("{:?}", c);

}

pub fn collect_strings(file_path: &str) -> Vec<String> {

    let reader = fasta::Reader::from_file(file_path.to_string()).unwrap();
    let mut string_vec = Vec::new();
    for (i, mut result) in enumerate(reader.records()) {
        let mut rec = result.unwrap();

        string_vec.push(rec.seq_string())
    }
    string_vec
}