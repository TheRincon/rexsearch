


const DUST_LEVEL: i32 = 20;  // original was 20
const DUST_WORD: i32 = 3;
const DUST_WINDOW: i32 = 64; // original is 64
const DUST_WINDOW_2: i32 = DUST_WINDOW >> 1;
const WORD_COUNT: i32 = 1 << ( DUST_WORD << 1 );
static BIT_MASK: i32 = WORD_COUNT - 1;

fn wo(len: i32, m: &mut [char], beg: &mut i32, end: &mut i32) -> i32 {

    let l1 = len - DUST_WORD + 1 - 5;
    if l1 < 0 { return 0 };
    let chrmap = [0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
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

    let (mut bestv, mut besti, mut bestj) = (0,0,0);
    let mut  words = [0; 64];
    let mut word: i32 = 0;

    for j in 0..len {
        word <<= 2;
        word |= chrmap[m[j as usize] as usize];
        words[j as usize] = word & BIT_MASK;
    }

    for i in 0..l1 {
        let mut counts = [0; 64];
        let mut sum: i32 = 0;

        for j in 2 .. len - i {
            word = words[i as usize + j as usize];
            let mut c = counts[word as usize];
            if c != 0 {
                sum += c;
                let mut v = 10 * sum / j;
                if v > bestv
                    {
                        bestv = v;
                        besti = i;
                        bestj = j;
                    }
            }
            counts[word as usize] += 1;
        }
    }
    let x = besti;
    let y = bestj;
    *beg = x;
    *end = x + y;
    return bestv;
}

pub fn dust(m: &mut [char], hardmask: bool) {

    let len = m.len() as i32;
    let (mut a, mut b) = (0,0);
    let mut s: Vec<char> = vec!['m'; len as usize];
    s.clone_from_slice(&m);
    if !hardmask {
        /* convert sequence to upper case unless hardmask in effect */
        // for i in 0..len {
            // m[i] = toupper(m[i])
        // };
        // m[len] = 0;  // what is this?
    }
    let mut i = 0;
    while i < len {
        let l = if len > i + DUST_WINDOW { DUST_WINDOW } else { len-i };
        let v = wo(l, &mut s[i as usize..], &mut a, &mut b);
        if v > DUST_LEVEL {
            if hardmask {
                for j in (a+i) as usize..(b + i + 1 ) as usize {
                    m[j] = 'N';
                }
            } else {
                for j in (a + i)..(b + i + 1) {
                    match m[j as usize] {
                        'A' | 'a' => m[j as usize] = 'a',
                        'C' | 'c' => m[j as usize] = 'c',
                        'G' | 'g' => m[j as usize] = 'g',
                        'T' | 't' => m[j as usize] = 't',
                        _ => m[j as usize] = 'z'
                    }
                }
            }
            if b < DUST_WINDOW_2 { i += DUST_WINDOW_2 - b };
        }
        i += DUST_WINDOW_2;
    }
}