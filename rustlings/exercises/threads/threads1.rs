// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });
    let status_shared = status.clone();

    // In Chapter 13, we mentioned we can use the move keyword
    // before the parameter list of a closure to force the closure
    // to take ownership of the values it uses in the environment.
    // This technique is especially useful when creating new threads
    // in order to transfer ownership of values from one thread to another.
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            
            *status_shared.jobs_completed.lock().unwrap() += 1;
        }
    });
    while *status.jobs_completed.lock().unwrap() < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
