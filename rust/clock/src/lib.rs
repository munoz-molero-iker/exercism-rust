use std::fmt::Debug;

pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let equal_minutes: bool = self.minutes == other.minutes;
        let equal_hours: bool = self.hours == other.hours;
        equal_minutes && equal_hours
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes: i32 = ((hours * 60) + minutes).rem_euclid(1440);
        let remaining_minutes: i32 = total_minutes.rem_euclid(60);
        let remaining_hours: i32 = total_minutes / 60;

        Clock {
            minutes: remaining_minutes,
            hours: remaining_hours,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
