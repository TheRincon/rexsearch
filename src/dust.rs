const DUST_LEVEL: i32 = 20;  // original was 20
const DUST_WORD: i32 = 3;
const DUST_WINDOW: i32 = 64; // original is 64
const DUST_WINDOW_2: i32 = 32;
const WORD_COUNT: i32 = 1 << ( DUST_WORD << 1 );
static BIT_MASK: i32 = WORD_COUNT - 1;

fn wo(len: i32, seq: &[char], begin: i32, end: i32) -> (i32, i32, i32) {

    let l1: i32 = len - DUST_WORD + 1 - 5; //    - 4 ?
    if l1 < 0 {
        return (0, 0, 0);
    }
    let g = [0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  1,  0,  0,  0,  2,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  3,  3,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  1,  0,  0,  0,  2,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  3,  3,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0];
    let mut counts = vec![0;WORD_COUNT as usize];
    let mut words = vec![0;DUST_WINDOW as usize];
    let mut word: i32 = 0;
    let mut bestv: i32 = 0;
    let mut bestj: i32 = 0;
    let mut bestk: i32 = 0;
    let mut a = begin;
    let mut b = end;

    for i in 0..len {
        word <<= 2;
        //word |= match seq[i as usize] {
        //    'T' | 't' | 'U' | 'u' => 3,
        //    'G' | 'g'             => 2,
        //    'T' | 't'             => 1,
        //    _                     => 0
        //};
        word |= g[seq[i as usize] as usize];
        // println!("word here is {:?}, ", word);
        words[i as usize] = word & BIT_MASK;
        // println!("{:?}", words[i as usize]);
    }
    // println!("words array is {:?}", words);
    for j in 0..l1 {
        let mut sum = 0;
        for k in 2 .. (len-j) {
            let p = j;
            let q = k;
            word = words[j as usize + k as usize];
            let c: i32 = counts[word as usize];
            // println!("{:?}", c);
            if c != 0 {
                sum += c;
                let mut v: i32 = 10 * sum / k;
                // println!("{:?}", v);
                if v > bestv {
                    bestv = v;
                    bestj = j;
                    bestk = k;
                }
            }
            counts[word as usize] += 1;
        }
    }
    a = bestj;
    b = bestj + bestk;
    println!("end of array is {:?}", b);
    return (bestv, a, b);
}

pub fn duster(seq: &str, hardmask: bool) -> Vec<char> {

    let n = seq.len() as i32;
    let mut a: i32 = 0;
    let mut b: i32 = 0;

    let mut ss: Vec<char> = seq.to_owned().clone().chars().collect();
    // let s = ss.clone();
    if !hardmask {
        seq.to_uppercase();
    }

    let mut i: i32 = 0;
    while i < n {
        let mut lm = if (n > (i + DUST_WINDOW)) { DUST_WINDOW } else { n - i };
        let za = a;
        let zb = b;
        let (v, a, b) = wo(lm, &ss[i as usize..], za, zb);
        println!("{:?}", &ss[i as usize..]);
        if v > DUST_LEVEL {
            if hardmask {
                // println!("{:?},{:?}",a,b);
                for j in (a + 1)..(b + 1) {
                    ss[j as usize] = 'N';
                }
            } else {
                // convert ss range a+1 .. b+1 to lowercase. Somehow.
            }
            if b < DUST_WINDOW_2 {
                i += DUST_WINDOW_2 - b;
            }
        }
        i += DUST_WINDOW_2;
    }
    ss
}