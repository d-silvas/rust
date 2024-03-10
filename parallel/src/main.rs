use is_prime::*;
use num_bigint::BigUint;
use rayon::prelude::*;

#[derive(Debug)]
struct Candidate {
    num: BigUint,
    is_prime: bool,
}

fn main() {
    let mut candidates: Vec<Candidate> = Vec::with_capacity(1000);
    let large_num: BigUint =
        // BigUint::parse_bytes(b"1000000000000000000000000000000000000000", 10).unwrap();
        BigUint::parse_bytes(b"10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10).unwrap();
    for i in 0_u32..=999_u32 {
        let next: BigUint = &large_num + &BigUint::from(i);
        candidates.push(Candidate {
            num: next,
            is_prime: false,
        })
    }
    candidates.par_iter_mut().for_each(|c: &mut Candidate| {
        c.is_prime = is_prime(c.num.to_string().as_str());
    });

    println!("{:#?}", candidates);
}
