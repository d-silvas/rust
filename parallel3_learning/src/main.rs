use std::{thread, time};

/**
 * In this program we want to check if waiting for one thread to finish (handle1)
 * locks other threads (handle2) that are not the main thread.
 */
fn main() {
    let handle1 = thread::spawn(|| {
        thread::sleep(time::Duration::from_millis(10000));
        println!("Thread 1 - FINISHED")
    });

    let handle2 = thread::spawn(|| {
        for i in 0..6 {
            thread::sleep(time::Duration::from_millis(1000));
            println!("Thread 2 - {i}");
        }
        println!("Thread 2 - FINISHED")
    });

    println!("Thread M - Waiting for handle1");
    handle1.join().unwrap();
    println!("Thread M - Wait for handle1 finished. Waiting for handle2");
    // If handle2 was blocked at this point, we should only see handle2 print
    // the first message (or no message). If handle2 is allowed to run, then
    // we will see all numbers printed by handle2.
    handle2.join().unwrap();
    println!("Thread M - Wait for handle2 finished");
}
