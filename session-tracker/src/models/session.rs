use std::time::Duration;

// Create a session to save the information, the goal duration.
pub struct Session {
    goal: String, // This is the goal of the session, it is a string that will be used to store the goal of the session.
    duration: Duration, // estimate duration for the goal
    actual_duration: Duration, // real duration of the session
    completed: bool, // this is a boolean that will be used to check if the session was completed or not
    in_time: bool, // according to the terms of the session
}

impl Session {
    pub fn new(goal: String, duration: Duration) -> Self {
        Session {
            goal,
            duration,
            actual_duration: Duration::from_secs(0),
            completed: false,
            in_time: false,
        }
    }

    pub fn in_time(&self) -> bool {
        // time is considered in time if the session is completed and the actual duration is less than or equal to the estimated duration plus 10 seconds
        self.completed && self.actual_duration <= self.duration + Duration::from_secs(10)
    }

    pub fn complete(&mut self, actual_duration: Duration){
        self.completed = true;
        self.actual_duration = actual_duration;
    }

}