
use rand::{thread_rng, Rng, sample};

pub fn shuffler(deck: &mut [&str]) {
    let mut rng = thread_rng();
    rng.shuffle( deck);
}

pub fn sub_sample(deck: &[String], num: usize) {
    let mut rng = thread_rng();
    let sample = sample(&mut rng, deck, num);
}
