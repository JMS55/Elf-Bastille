use crate::DELTA_TIME;
use std::time::Duration;

pub struct Timer {
    trigger_time: Duration,
    time_waited: Duration,
}

impl Timer {
    pub fn new(trigger_time: Duration, trigger_at_start: bool) -> Self {
        if trigger_at_start {
            Self {
                trigger_time,
                time_waited: trigger_time,
            }
        } else {
            Self {
                trigger_time,
                time_waited: Duration::from_secs(0),
            }
        }
    }

    pub fn triggered(&mut self) -> bool {
        self.time_waited += DELTA_TIME;
        if self.time_waited >= self.trigger_time {
            self.time_waited = Duration::from_secs(0);
            return true;
        }
        false
    }
}
