use chrono::{DateTime, Datelike, Days, Local, Timelike};
use std::cell::Cell;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Entry {
    command: String,
    schedule: Schedule,
    next_run: Cell<Option<DateTime<Local>>>,
}

impl Entry {
    pub fn new(command: String, schedule: Schedule) -> Self {
        Self {
            command,
            schedule,
            next_run: Cell::new(None),
        }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn next_run(&self, time: &DateTime<Local>) -> DateTime<Local> {
        if self.next_run.get().is_none() || self.next_run.get().unwrap() < *time {
            self.next_run.replace(Some(self.schedule.calc_next(time)));
        }

        self.next_run.get().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Schedule {
    minute: Constraint,
    hour: Constraint,
    month_day: Constraint,
    month: Constraint,
    week_day: Constraint,
}

impl Schedule {
    pub fn new(
        minute: Constraint,
        hour: Constraint,
        month_day: Constraint,
        month: Constraint,
        week_day: Constraint,
    ) -> Self {
        Self {
            minute,
            hour,
            month_day,
            month,
            week_day,
        }
    }

    pub fn calc_next(&self, time: &DateTime<Local>) -> DateTime<Local> {
        // First check if today is possible
        if self.month.satisfy(time.month())
            && self.month_day.satisfy(time.day())
            && self.week_day.satisfy(time.weekday().num_days_from_sunday())
        {
            if let Some(time) = self.first_in_day(time) {
                return time;
            }
        }

        // Figure out first day
        let mut max_days = 3650; // Assume we can find one in ten years
        let mut time = time
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        while max_days > 0 {
            max_days -= 1;
            time = time.checked_add_days(Days::new(1)).unwrap();

            if self.month.satisfy(time.month())
                && self.month_day.satisfy(time.day())
                && self.week_day.satisfy(time.weekday().num_days_from_sunday())
            {
                if let Some(time) = self.first_in_day(&time) {
                    return time;
                }
            }
        }

        panic!("Can't find a proper day in ten years")
    }

    fn first_in_day(&self, time: &DateTime<Local>) -> Option<DateTime<Local>> {
        let hour = time.hour();
        let mut minute = time.minute() + 1;
        if time.second() == 0 && time.nanosecond() == 0 {
            minute -= 1;
        }

        let time = time.with_second(0).unwrap().with_nanosecond(0).unwrap();

        // if this hour is possible
        if self.hour.satisfy(hour) {
            for minute in minute..60 {
                if self.minute.satisfy(minute) {
                    return Some(time.with_minute(minute).unwrap());
                }
            }
        }

        for hour in hour + 1..24 {
            if !self.hour.satisfy(hour) {
                continue;
            }
            for minute in 0..60 {
                if self.minute.satisfy(minute) {
                    return Some(time.with_hour(hour).unwrap().with_minute(minute).unwrap());
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct Constraint {
    valid: Vec<bool>,
}

impl Constraint {
    pub fn new(valid: Vec<bool>) -> Self {
        Self { valid }
    }

    pub fn satisfy(&self, value: u32) -> bool {
        self.valid[value as usize]
    }
}

pub fn path() -> PathBuf {
    let mut path = PathBuf::new();

    #[cfg(windows)]
    path.push(env::var("LOCALAPPDATA").unwrap());

    #[cfg(unix)]
    {
        path.push(env::var("HOME").unwrap());
        path.push(".config");
    }

    path.push("wincron");
    path.push("crontab");

    path
}
