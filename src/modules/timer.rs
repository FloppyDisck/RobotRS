use bevy_ecs::prelude::*;
use std::time::SystemTime;

#[derive(Component, Debug)]
pub struct Timer {
    duration: Duration,
    state: State,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            state: State::Wait,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state == State::Ready
    }

    pub fn update(&mut self) {
        match self.state {
            State::Wait => {
                self.state = State::Waiting {
                    time: SystemTime::now(),
                }
            }
            State::Waiting { time } => {
                let elapsed = SystemTime::now().duration_since(time).unwrap();
                if match self.duration {
                    Duration::Millis(duration) => elapsed.as_millis() >= duration,
                    Duration::Micros(duration) => elapsed.as_micros() >= duration,
                    Duration::Nanos(duration) => elapsed.as_nanos() >= duration,
                    Duration::Secs(duration) => elapsed.as_secs() >= duration,
                } {
                    self.state = State::Ready;
                }
            }
            State::Ready => {
                self.state = State::Wait;
            }
        }
    }
}

#[derive(Debug)]
pub enum Duration {
    Millis(u128),
    Micros(u128),
    Nanos(u128),
    Secs(u64),
}

#[derive(PartialEq, Debug)]
pub enum State {
    Wait,                         // Ask for the timer to start waiting
    Waiting { time: SystemTime }, // Will check when time is ready
    Ready,                        // Notifies that the timer is ready
}

pub(crate) fn timer_system(mut query: Query<&mut Timer>) {
    for mut timer in &mut query {
        if !timer.is_ready() {
            timer.update();
        }
    }
}

/// Automatically checks for a timer variable and skips the current loop if true
#[macro_export]
macro_rules! check_timer {
    ($timer:ident) => {
        if let Some(mut timer) = $timer {
            if timer.is_ready() {
                timer.update();
            } else {
                continue;
            }
        }
    };
}
