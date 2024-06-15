use env_logger::Env;
use is_prime::*;
use log::debug;
use num_bigint::BigUint;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Debug)]
struct Candidate {
    num: BigUint,
    is_prime: bool,
}

// impl Copy for Candidate {}

// impl Clone for Candidate {
//     fn clone(&self)-> Candidate {
//         return Candidate {
//             num: self.num.clone(),
//             is_prime: self.is_prime,
//         }
//     }
// }

// const NUM_THREADS: usize = 10;
// const NUM_CANDIDATES: usize = 100;
// const NUM_CANDIDATES_PER_THREAD: usize = NUM_CANDIDATES / NUM_THREADS;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let start = Instant::now();

    let num_iter: u32 = 100;
    let mut candidates: Vec<Candidate> = Vec::with_capacity(num_iter as usize);
    // let mut candidates_matrix = [[Candidate { num: BigUint::zero(), is_prime:false }; NUM_CANDIDATES_PER_THREAD]; NUM_THREADS]
    let large_num: BigUint =
        // BigUint::parse_bytes(b"1000000000000000000000000000000000000000", 10).unwrap();
        BigUint::parse_bytes(b"10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10).unwrap();
    debug!("Create vars: {:.2?}", start.elapsed());

    let before = Instant::now();
    for i in 0_u32..num_iter {
        let next: BigUint = &large_num + &BigUint::from(i);
        candidates.push(Candidate {
            num: next,
            is_prime: false,
        })
    }
    debug!("Create cand: {:.2?}", before.elapsed());

    let before = Instant::now();
    candidates.par_iter_mut().for_each(|c: &mut Candidate| {
        c.is_prime = is_prime(&c.num.to_string());
    });
    debug!("Prim checks: {:.2?}", before.elapsed());

    debug!("TOTAL: {:.2?}", start.elapsed());

    // println!("{:#?}", candidates);
}
