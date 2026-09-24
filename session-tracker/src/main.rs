mod models;
mod modules;

use std::time::Duration;
use models::session::Session;
use modules::time_session;
// we import the input module to read the input from the user and the time_session module to take the time in minutes.
use std::io;


fn main() {
    let duration = Duration::from_secs(60); // This is the duration of the session, it is a duration of 60 seconds that will be used to check if the user has entered any input.
    let goal = String::from("Complete the task"); // This is the goal of the session, it is a string that will be used to store the goal of the session.
    let mut session = Session::new(goal, duration); // This is the session that will be created to store the information of the session.    let time = time_session::take_time_minutes();
    let time = time_session::take_time_minutes();

    session.complete(time); 
    println!("Session lasted {} seconds", time.as_secs());
    println!("Completed in time: {}", session.in_time());

    
}