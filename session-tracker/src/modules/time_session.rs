
use std::io; 
use std::sync::mpsc; 
use std::thread; 
use std::time::{Duration, Instant};
use std::sync::mpsc::Receiver;

// Import	    Purpose
// io	        Read keyboard input.
// mpsc	        Create a channel for communication between threads.
// Receiver	    The type representing the receiving end of a channel.
// thread	    Start threads and pause execution.
// Duration	    Represent a length of time.

fn spam() -> Receiver<String> {
    let (tx, rx) = mpsc::channel(); // create a channel for communication between threads
    // || introduces a closure: an anonymous function. Empty || means it takes no
    thread::spawn(move || loop {
        let mut input = String::new(); // this is a mutable string that will be used to store the input from the user
        
        if io::stdin().read_line(&mut input).is_ok() // this is a check to see if the input was read successfully
        {
            let _ =tx.send(input); // this allow use to send the input to the main thread
        }
    });
    rx // This is the second thread that is created to read input from the user and send it to the main thread. The main thread will then receive the input and process it accordingly.
}


// Method	  | Behavior
// recv()	  : Waits for a message if the channel is empty and connected.
// try_recv() :	Returns immediately, whether a message is available or not.
pub fn take_time_minutes() -> Duration {
    let stdin_channel = spam(); // this is a function that will create a channel for communication between threads
    let mut timer = Instant::now();
    let interval = Duration::from_secs(1); // this is a duration of 5 seconds that will be used to check if the user has entered any input
    let mut timer_duration = Duration::from_secs(0); 

    loop {
        // Take the time elapsed since the last time the timer was reset and check if it is greater than or equal to the interval. If it is, print a message to the user and reset the timer.
        // we are counting the time in seconds, so we are using Duration::from_secs(1) to create a duration of 1 second. We are also using Duration::from_secs(0) to create a duration of 0 seconds that will be used to keep track of the total time elapsed.
        if timer.elapsed() >= interval {
            println!("Enter to stop session...");
            timer = Instant::now(); 
            timer_duration = Duration::from_secs(timer_duration.as_secs() + 1);

            println!("{} seconds", timer_duration.as_secs());
        }
        match stdin_channel.try_recv(){
            // ok : is a method that will return the value of the input 
            // if it is available, or an error if it is not.
            Ok(input) => {
                if input.trim().is_empty() {
                    println!("Empty Enter detected");
                    return timer_duration;
                }
            }

            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {}
        }
        
    }
} 

