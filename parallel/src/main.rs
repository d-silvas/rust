use rayon::prelude::*;

#[derive(Debug)]
struct Candidate {
    num: i32,
    is_even: bool,
}

fn main() {
    let mut candidates: Vec<Candidate> = Vec::with_capacity(1000);
    for i in 0..=999 {
        candidates.push(Candidate {
            num: i,
            is_even: false,
        })
    }
    candidates.par_iter().for_each(|c: &mut Candidate| {
        c.num = 3;
        println!("{:?}", c.num)
    });
    // println!("{:?}", candidates);
}
