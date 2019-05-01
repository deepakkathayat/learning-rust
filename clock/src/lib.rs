use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MAX_HOURS: i32 = 24;
const MAX_MINUTES: i32 = 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let h = (hours as f32 + (minutes as f32 / MAX_MINUTES as f32).floor()) as i32 % MAX_HOURS;
        let m = minutes % MAX_MINUTES;
        Clock {
            hours: if h >= 0 { h } else { h + MAX_HOURS },
            minutes: if m >= 0 { m } else { m + MAX_MINUTES },
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
