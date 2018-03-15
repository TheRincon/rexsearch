const DUST_LEVEL: i64 = 20;
const DUST_WORD: i64 = 3;
const DUST_WINDOW: i64 = 64; // original is 64
const DUST_WINDOW_2: i64 = DUST_WINDOW >> 1;
const WORD_COUNT: i64 = 1 << ( DUST_WORD << 1 );
static BIT_MASK: i64 = WORD_COUNT - 1;

fn wo(len: i64, seq: &Vec<char>, begin: &i64, end: &i64) -> (i64, i64, i64) {

    let mut bestv: i64 = 0;
    let mut bestj: i64 = 0;
    let mut bestk: i64 = 0;
    let l1: i64 = len - DUST_WORD + 1 - 5; //    - 4 ?

    if l1 < 0 {
        return (0, 0, 0);
    }
    let counts = vec![0;WORD_COUNT as usize];
    let mut words = vec![0;DUST_WINDOW as usize];
    let mut word: i64 = 0;
    for i in 0..len {
        word <<= 2;
        word |= match seq[i as usize] {
            'T' | 't' | 'U' | 'u' => 3,
            'G' | 'g'             => 2,
            'T' | 't'             => 1,
            _                     => 0
        };
        words[i as usize] = word & BIT_MASK;
    }
    for j in 0..l1 {
        let mut sum: i64 = 0;
        for k in (DUST_WORD-1) .. (len-1) {
            word = words[j as usize + k as usize];
            let c: i64 = counts[word as usize];           // find out what is this in original
            if c > 0 {                                    // definitely probably wrong
                sum += c;
                let mut v = 10 * sum / k;
                if v > bestv {
                    bestv = v;
                    bestj = j;
                    bestk = k;
                }
            }
            counts[word as usize] + 1;                             // Find out what this is (moving address?)
        }
    }
    return (bestv, bestj, bestk + bestj);
}

pub fn duster(seq: &str, hardmask: bool) -> Vec<char> {

    let n = seq.len() as i64;
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let mut ss: Vec<char> = seq.to_owned().clone().chars().collect();
    let s = ss.clone();
    let mut i: i64 = 0;
    if !hardmask {
        seq.to_uppercase();
    }
    while i < n {
        let mut lm = if (n > (i + DUST_WINDOW)) { DUST_WINDOW } else { n - i };
        let (v, a, b) = wo(lm, &s, &mut a, &mut b);
        if v > DUST_LEVEL {
            if hardmask {
                println!("{:?},{:?}",a,b);
                for j in (a + 1)..(b + 1) {
                    ss[j as usize] = 'N';
                }
            } else {
                // convert ss range a+1 .. b+1 to lowercase. Somehow.
            }
        }
        if b < DUST_WINDOW_2 {
            i += DUST_WINDOW_2 - b as i64;
        }
        i += DUST_WINDOW_2;
    }
    ss
}