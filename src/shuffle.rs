
use rand::{thread_rng, Rng, sample};

pub fn shuffler(deck: &mut [&str]) {
    let mut rng = thread_rng();
    rng.shuffle( deck);
}

pub fn sub_sample(deck: &[String], num: usize) {
    let mut rng = thread_rng();
    let sample = sample(&mut rng, deck, num);
}

pub fn percentage_sub_sample(deck: &[String], percentage: i32) {
    if percentage < 0 || percentage > 100 {
        panic!("Percentage must be between 0 and 100");
    }
    let act = percentage as usize * (deck.len() / 100);          // is this right?
    let mut rng = thread_rng();
    let sample = sample(&mut rng, deck, act);
}
