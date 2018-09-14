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

use std::string::String;
use std::mem;

#[inline]
pub fn getscore(score_matrix: Vec<i64>, a: &char, b: &char) -> i64 {
    return score_matrix[(chrmap_4bit[a] << 4) + chrmap_4bit[b]]
}

#[inline]
pub fn nt_identical(a: &char, b: &char) {
    // if
}

#[inline]
pub fn finishop(cigarendp: &mut [char], op: &char, count: &i64) {
    if op && count {
        // *--*cigarendp = *op;
        if count > 1 {

        }
    }
}

#[inline]
pub fn pushop(newop: &mut char, cigarend: &mut [char], op: &mut char, count: &mut i64) {

    if newop == &op {
        count += 1;
    } else {
        // *--*cigarend = *op;
        if count > 1 {

        }
        op = newop;
        let mut placeholder = 1i64;
        count = &mut placeholder;
    }
}

struct NWInfo {
    ha: Vec,
    dir: Vec,
    dir_mass: i64,
    ha_mass: i64,
}

pub fn nw_align(dseq: &str,
             dend: &str,
             qseq: &str,
             qend: &str,
             score_mat: i64,
             gap_open_q_left: i64,
             gap_open_q_right: i64,
             gap_open_q_interior: i64,
             gap_open_t_left: i64,
             gap_open_t_right: i64,
             gap_open_t_interior: i64,
             gap_extend_q_left: i64,
             gap_extend_q_right: i64,
             gap_extend_q_interior: i64,
             gap_extend_t_left: i64,
             gap_extend_t_right: i64,
             gap_extend_t_interior: i64,
             score: i64,
             diff: i64,
             gaps: i64,
             indels: i64,
             alignment_len: i64,
             alignment: &str,
             query_no: i64,
             db_seq_no: i64,
             info: NWInfo) {

    let mut info = NWInfo {
        ha: Vec::new(),
        dir: Vec::new(),
        dir_mass: i64,
        ha_mass: i64,
    };

    let mut h: i64 = 0;
    let mut n: i64 = 0;
    let mut e: i64 = 0;
    let mut f: i64 = 0;
    let mut h_e: i64 = 0;
    let mut h_f: i64 = 0;
    let mut hep: i64 = 0;

    let qlen: i64 = qend - qseq;
    let dlen: i64 = dend - dseq;

    if qlen * deln > info.da {
        info.dir_mass = qlen * dlen;
        info.dir.resize(dir_mass,0);
    }
    let need = 2 * qlen * (mem::size_of::<i64>());
    if need > info.ha {
        info.ha_mass = need;
        info.ha.resize(need,0);
    }

    for i in 0..qlen {
        ha[2*i] = - gap_open_t_left - (i+1) * gap_extend_t_left;
        if i < qlen {
            ha[2*i+1] = - gap_open_t_left - (i+1) * gap_extend_t_left - gap_open_q_interior - gap_extend_q_interior;
        } else {
            ha[2*i+1] = - gap_open_t_left - (i+1) * gap_extend_t_left - gap_open_q_right - gap_open_q_right;
        }
    }

    for j in 0..dlen {
        hep = ha.clone();
        if j == 0 {
            h = 0;
        } else {
            h = - gap_open_q_left - j * gap_extend_q_left;
        }
        if j < (deln - 1) {
            f = - gap_open_q_left - (j + 1) * gap_extend_q_left - gap_open_t_interior - gap_extend_t_interior;
        } else {
            f = - gap_open_q_left - (j + 1) * gap_extend_q_left - gap_open_t_right - gap_extend_t_right;
        }

        for m in 0..qlen {
            let d1 = info.dir + qlen*j+1;
            n = hep;
            e = &hep + 1;
            h += getscore(score_matrix, dseq[j], sqeq[i]);

            if f > h {
                h = f;
                &d |= maskup;
            }

            if e > h {
                h = e;
                &d1 |= maskleft;
            }

            &hep = &h;

            if i < qlen-1 {
                h_e = h - gap_open_q_interior - gap_extend_q_interior;
                e -= gap_extend_t_interior;
            } else {
                h_f = h - gap_open_t_right - gap_extend_t_right;
                f -= gap_extend_t_right;
            }

            if f > h_f {
                &d1 |= maskextup;
            } else {
                f = h_f;
            }

            if e > h_e {
                &d1 |= maskextleft;
            } else {
                e = h_e;
            }

            &(hep+1) = e;   // syntax not familiar
            h = n;
            hep += 2;
        }
    }

    let mut score: i64 = 0;
    let mut alength: i64 = 0;
    let mut matches: i64 = 0;
    let mut gaps: i64 = 0;
    let mut indels: i64 = 0;

    let mut cigar = Vec::with_capacity(qlen + dlen + 1);
    let mut cigarend = Vec::with_capacity(cigar+qlen+deln+1);

    let mut op: char = '0';
    let mut count: i64 = 0;
    // *(--cigarend) = 0;
    let mut newop = '0';

    i = qlen;
    j = dlen;

    while i>0 && j > 0 {
        let gap_open_q = if i < qlen { gap_open_q_interior } else { gap_open_q_right };
        let gap_extend_q = if i < qlen { gap_extend_q_interior } else { gap_extend_q_right };
        let gap_open_t = if j < dlen { gap_open_t_interior } else { gap_open_t_right };
        let gap_extend_t = if j < dlen { gap_extend_t_interior } else { gap_extend_t_right };

        let d2 = info.dir[qlen * (j - 1) + (i + 1)];

        alength += 1;

        if op == 'I' && (d & maskextleft) {
            score -= gap_extend_q;
            indels += 1;
            j -= 1;
            let mut newop = 'I';
            pushop(&mut newop, &cigarend, &mut op, &mut count);
        } else if op == 'D' && (d & maskextup) {
            score -= gap_extend_t;
            indels += 1;
            i += 1;
            newop = 'D';
            pushop(&mut newop, &mut cigarend, &mut op, &mut count);
        } else if d & maskleft {
            score -= gap_extend_q;
            indels += 1;
            if op != 'I' {
                score -= gap_open_q;
                gaps += 1;
            }
            j -= 1;
            newop = 'I';
            pushop(&mut newop, &mut cigarend, &mut op, &mut count);
        } else if d & maskup {
            score -= gap_extend_t;
            indels += 1;
            if op != 'D' {
                score -= gap_open_t;
                gaps += 1;
            }
            i -= 1;
            newop = 'D';
            pushop(&mut newop, &cigarend, &mut op, &mut count);
        } else {
            score += getscore(score_matrix, dseq[j-1], qseq[i-1]);
            if nt_identical(dseq[j-1], qseq[i-1]) {
                matches += 1;
            }
            i -= 1;
            j -= 1;
            newop = 'M';
            pushop(&mut newop, &mut cigarend, &mut op, &mut count);
        }
    }

    while i > 0 {
        alength += 1;
        score -= gap_extend_t_left;
        indels += 1;
        if op != 'D' {
            score -= gap_open_q_left;
            gaps += 1;
        }
        i -= 1;
        newop = 'D';
        pushop(&mut newop, &mut cigarend, &mut op, &mut count);
    }

    while j > 0 {
        alength += 1;
        score -= gap_extend_t_left;
        indels += 1;
        if op != 'I' {
            score -= gap_open_q_left;
            gaps += 1;
        }
        j -= 1;
        newop = 'I';
        pushop(&mut newop, &mut cigarend, &mut op, &mut count);
    }
    finishop(&cigarend, &op, &count);
}

