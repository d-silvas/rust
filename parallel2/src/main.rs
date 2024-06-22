use chrono::Local;
use env_logger::Env;
use is_prime::*;
use log::{debug, info};
use num_bigint::BigUint;
use std::fs::File;
use std::io::Write;
use std::thread::{self, JoinHandle};
use std::time::Instant;

const NUM_THREADS: usize = 100;
const NUM_CANDIDATES_PER_THREAD: usize = 100;

fn main() {
    let log_target = Box::new(File::create("result.log").expect("Can't create file"));
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .target(env_logger::Target::Pipe(log_target))
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    let start = Instant::now();
    let large_num: BigUint =
        BigUint::parse_bytes(b"10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10).unwrap();

    let mut thread_handles: Vec<JoinHandle<()>> = Vec::with_capacity(NUM_THREADS);
    for i in 0..NUM_THREADS {
        let large_num_clone = large_num.clone();
        let handle = thread::spawn(move || {
            for j in 0..NUM_CANDIDATES_PER_THREAD {
                let candidate_num = &large_num_clone + (i * NUM_CANDIDATES_PER_THREAD) + j;
                let is_candidate_num_prime = is_prime(&candidate_num.to_string());
                info!("{is_candidate_num_prime} :: {candidate_num}");
            }
        });
        thread_handles.push(handle);
    }

    for h in thread_handles {
        h.join().unwrap();
    }

    debug!("TOTAL: {:.2?}", start.elapsed());
}
