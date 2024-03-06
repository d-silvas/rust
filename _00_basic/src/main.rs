use std::io;
use std::time::Instant;

fn main() {
    /* VARIABLES **/
    let mut x: u128 = 4;
    println!("Hey {}", x);

    {
        // Inside a block we can use "outer" variables
        println!("Hey {}", x);
        // And we can also modify them (now x=5)
        x = x + 1;
        println!("Hey {}", x);
        // But if we redefine them, the new variable is
        // scoped to this block.
        // Once outside the block, "x" is 5 again
        let x: &str = "ayayay";
        println!("Hey {}", x);
    }

    x = x + 1;
    println!("Hey {}", x);

    // We can "override" previous values. Note that
    // the new one is not mutable
    let x: i32 = 8;
    println!("Hey {}", x);

    /* CONSTANTS **/
    const SECONDS_IN_MINUTE: i8 = 60;
    println!("Secs in minute: {}", SECONDS_IN_MINUTE);

    types();
    read_input();
    strings();
}

fn types() {
    println!("---TYPES---");
    let tuple: (u8, bool, char, &str) = (1, true, 's', "hi");
    println!(
        "Tuple elements: {}, {}, {}, {}",
        tuple.0, tuple.1, tuple.2, tuple.3
    );

    // EULER PROBLEM 1
    let now = Instant::now();
    let mut sum: u128 = 0;
    let mut prev: u128 = 1; // Not even
    let mut curr: u128 = 1; // Not even

    for _n in 1..100 {
        let temp: u128 = prev;
        prev = curr;
        curr = curr + temp;
        if curr > 4_000_000 {
            break;
        }
        if is_even(curr) {
            sum += curr;
        }
    }
    let elapsed = now.elapsed();
    println!("ELAPSED: {:.2?}", elapsed);

    println!("{}, {}", prev, curr);
    println!("SUM {}", sum);
}

fn is_even(num: u128) -> bool {
    num % 2 == 0
}

fn read_input() {
    println!("---- INPUT ----");
    println!("Please input whatever");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("You told me: {}", input);

    let mut hopefully_a_number: String = String::new();
    println!("Please input a number");
    io::stdin()
        .read_line(&mut hopefully_a_number)
        .expect("Brah");
    let the_number: i32 = hopefully_a_number.trim().parse().unwrap();
    println!("You gave me: {}", the_number);
}

fn strings() {
    let string_or_what = "hoihoi";
}
