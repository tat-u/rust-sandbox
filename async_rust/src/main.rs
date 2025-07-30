use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static RUNNING: AtomicBool = AtomicBool::new(false);

fn some_fn_takes_a_long_time() -> () {
    println!("Function is running...");
    thread::sleep(Duration::from_secs(3));
    println!("Function finished.");
}

fn main() -> () {
    if RUNNING.swap(true, Ordering::SeqCst) {
        println!("Another instance is already running.");
        return;
    }

    println!("Starting some threads...");

    let mut handles = vec![];

    for _ in 0..5 {
        let handle = thread::spawn(move || {
            some_fn_takes_a_long_time();
        });
        handles.push(handle);
    }

    println!("Waiting for all the threads to finish...");

    for handle in handles {
        handle.join().unwrap();
    }

    RUNNING.store(false, Ordering::SeqCst);

    println!("All threads have completed execution.");
}
