use std::string::String;
use std::mem;

struct NWInfo {
    ha: Vec,
    dir: Vec,
    dir_mass: i64,
    ha_mass: i64,
}

pub fn align(dseq: &str,
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
        ha: 0,
        dir: "a",
        dir_mass: 0,
        ha_mass: 0,
    };

    let mut h: i64 = 0;
    let mut n: i64 = 0;
    let mut e: i64 = 0;
    let mut f: i64 = 0;
    let mut h_e: i64 = 0;
    let mut h_f: i64 = 0;

    let qlen: usize = qend - qseq;
    let dlen: usize = dend - dseq;

    if qlen * deln > info.da {
        info.dir_mass = qlen * dlen;
        info.dir.resize(dir_mass,0);
    }
    let need = 2 * qlen * (mem::size_of::<i64>());
    if (need > info.ha) {
        info.ha_mass = need;
        info.ha.resize(need,0);
    }

    for i in 0..qlen {
        ha[2*i] = - gap_open_t_left - (i+1) * gap_extend_t_left;
        if i < qlen {
            ha[2*i+1] = - gap_open_t_left - (i+1) * gap_extend_t_left - gap_open_q_interior - gap_extend_q_interior; // probably not even faster than (-2 * gap_ext..)
        } else {
            ha[2*i+1] = - gap_open_t_left - (i+1) * gap_extend_t_left - gap_open_q_right - gap_open_q_right;
        }
    }

    for j in 0..dlen {
        hep = ha.clone();
        if j == 0 {
            h = 0;
        } else {
            h = - gap_open_q_left - j *
        }
    }
}