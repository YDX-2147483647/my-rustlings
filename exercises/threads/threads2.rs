// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

// Note:
// I have to `join` on all the handles!
// If I let the threads sleep for different durations (eg. 250 * (i+1)), I will no longer get ten `10`s.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    let mut handles = vec![];
    for i in 0..10 {
        let status_shared = status.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // You must take an action before you update a shared value
            let mut status_shared = status_shared.lock().unwrap();
            status_shared.jobs_completed += 1;
            println!("jobs completed {} (from thread #{i})", status_shared.jobs_completed);
        });
        handles.push(handle);
    }

    println!("jobs completed {} (before you join)", status.lock().unwrap().jobs_completed);

    for handle in handles {
        handle.join().unwrap();
        // Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
