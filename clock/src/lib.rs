use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_min = (hours * 60 + minutes) % 1440;
        println!("{}", total_min);
        
        if total_min < 0 {
            return Clock{hour: 24 - (total_min as f64 / 60.0).floor().abs() as i32, minute: (60 - total_min.abs() % 60) % 60 };    
        }

        Clock { hour: (total_min as f64 / 60.0).floor() as i32, minute: total_min % 60 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_min = (self.hour * 60 + self.minute + minutes) % 1440;
        println!("{}", total_min);

        if total_min < 0 {
            return Clock{hour: 24 - (total_min as f64 / 60.0).floor().abs() as i32, minute: (60 - total_min.abs() % 60) % 60 };    
        }

        Clock { hour: (total_min as f64 / 60.0).floor() as i32, minute: total_min % 60 }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}