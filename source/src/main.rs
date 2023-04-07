
use std::time::Duration;
use std::thread;
use std::io;
use std::sync::{Mutex, Arc};

fn main() {
    //we create the object we are trying to modify in the threads
    let mut unique_object = Arc::new(Mutex::new(1));

    'main: loop {
        // copy the Arc to have a safe reference counter that works with threads 
        let mut clone1 = Arc::clone(&unique_object);
        // moving the copy into the new thread
        let handle1 = thread::spawn(move || {
            // making change, asking for exclusive permission to the Mutex
            println!("First Thread");
            *clone1.lock().unwrap() += 1; 

        });
        let mut clone2 = Arc::clone(&unique_object);
        let handle2 = thread::spawn(move || {
            println!("Second Thread");
            *clone2.lock().unwrap() += 1;
        });



        println!("{:?}", unique_object);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        if input.trim().to_lowercase() == "q" {
            break 'main
        }
    }
}
