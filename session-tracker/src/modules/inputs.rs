pub mod inputs_for_session {
    use std::{io, time::Duration};
    pub enum input_session {
        Goal(String),
        Duration(Duration),
    }
    
    
    
    


    impl input_session {
        pub fn goals(&self) -> String {
            match self {
                input_session::Goal(goal) => goal.clone(),
                _ => String::from("No goal provided"),
            }
        }
    
        pub fn duration(&self) -> Option<Duration> {
            match self {
                input_session::Duration(duration) => Some(*duration),
                _ => None,
            }
        }
    } 

    pub fn inputs () -> input_session {
        // we are going to assing the goal through a input from the user, we will use the input module to read the input from the user and assign it to the goal variable.
        let mut goal = String::new();
        println!("Enter your goal for this session: ");
        io::stdin().read_line(&mut goal).expect("Failed to read line");
        // and assing the duration to the session, we will use the time_session module to take the time in minutes and assign it to the duration variable.
        let mut input = String::new();
        let mut input_duration = input.trim().parse::<u64>().unwrap_or(0);
        println!("Enter the duration of the session in minutes: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let duration:Option<Duration> = Some(Duration::from_secs(input_duration * 60)); // we are going to convert the input from minutes to seconds and assign it to the duration variable.

        // create a enum with input and goal
        let input_to = input_session::Goal(goal);
        let input_to = input_session::Duration(duration.unwrap_or(Duration::from_secs(0))); // we are going to convert the input from minutes to seconds and assign it to the duration variable.
        
        input_to

    }
}