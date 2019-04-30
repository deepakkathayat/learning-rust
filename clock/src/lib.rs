#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let h = (hours as f32 + ((minutes / 60) as f32).floor()) as i32 % 24;
        let m = minutes % 60;
        let clock = Clock {
            hours: if h >= 0 { h } else { h + 24 },
            minutes: if m >= 0 { m } else { m + 60 },
        };
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
