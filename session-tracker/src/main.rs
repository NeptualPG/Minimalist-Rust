mod models;
mod modules;

use models::session::Session;
use modules::time_session;
// we import the input module to read the input from the user and the time_session module to take the time in minutes.
use modules::inputs::inputs_for_session;


fn main() {
    /* */
    let inputs = inputs_for_session::inputs();  
    let mut session = Session::new(inputs.goals(), inputs.duration()); // This is the session that will be created to store the information of the session.    let time = time_session::take_time_minutes();
    let time = time_session::take_time_minutes();
    
    session.complete(time); 
    println!("Session lasted {} seconds", time.as_secs());
    println!("Completed in time: {}", session.in_time());

    
}