use rand::{seq::IteratorRandom, thread_rng}; // 0.6.1

fn main() {
    let mut rng = thread_rng();
    let v = vec![1, 2, 3, 4, 5];
    let sample = v.iter().choose_multiple(&mut rng, 2);

    println!("{:?}", sample);
}
