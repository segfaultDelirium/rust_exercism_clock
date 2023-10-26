use std::fmt::Display;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn euclid_modulo(x: i32, n: i32) -> i32 {
    let res = x % n;
    if res < 0 {
        res + n
    } else {
        res
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_in_correct_range = euclid_modulo(minutes, 60);
        let additional_hours_from_minutes = minutes / 60;
        let hour_to_remove_when_negative_minutes = if minutes < 0 && minutes_in_correct_range != 0 {
            1
        } else {
            0
        };
        let hours_in_correct_range = euclid_modulo(
            additional_hours_from_minutes + hours - hour_to_remove_when_negative_minutes,
            24,
        );
        Clock {
            hours: hours_in_correct_range,
            minutes: minutes_in_correct_range,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours_to_add = minutes / 60;
        let minutes_to_add = minutes % 60;

        let added_hours = self.hours + hours_to_add;
        let added_minutes = self.minutes + minutes_to_add;

        Clock::new(added_hours, added_minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.hours, self.minutes))
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
