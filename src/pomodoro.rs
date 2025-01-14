use std::time::Duration;

use crate::{counter::Counter, timer::Timer};

pub struct Pomodoro {
    timer: Timer,
    mode: Mode,
    session: u64,

    work_time: Duration,
    break_time: Duration,
    long_break_time: Duration,
}

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Work,
    Break,
    LongBreak,
}

impl Pomodoro {
    pub fn new(
        work_time: Duration,
        break_time: Duration,
        long_break_time: Duration
        ) -> Self {
        Self {
            timer: Timer::new(work_time),
            mode: Mode::Work,
            session: 1,

            work_time,
            break_time,
            long_break_time,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn check_next_mode(&self) -> Mode {
        match self.mode {
            Mode::Work => {
                if self.session % 4 == 0 {
                    Mode::LongBreak
                } else {
                    Mode::Break
                }
            }
            Mode::Break | Mode::LongBreak => Mode::Work,
        }
    }

    pub fn next_mode(&mut self) {
        match self.check_next_mode() {
            Mode::Work => self.work_mode(),
            Mode::Break => self.break_mode(),
            Mode::LongBreak => self.long_break_mode(),
        }
    }

    pub fn session(&self) -> u64 {
        self.session
    }

    fn work_mode(&mut self) {
        self.session += 1;
        self.mode = Mode::Work;
        self.timer = Timer::new(self.work_time);
    }

    fn break_mode(&mut self) {
        self.mode = Mode::Break;
        self.timer = Timer::new(self.break_time);
    }

    fn long_break_mode(&mut self) {
        self.mode = Mode::LongBreak;
        self.timer = Timer::new(self.long_break_time);
    }
}

impl Counter for Pomodoro {
    fn has_ended(&self) -> bool {
        self.timer.has_ended()
    }

    fn is_running(&self) -> bool {
        self.timer.is_running()
    }

    fn elapsed(&self) -> Duration {
        self.timer.elapsed()
    }

    fn pause(&mut self) {
        self.timer.pause()
    }

    fn resume(&mut self) {
        self.timer.resume()
    }

    fn end_count(&mut self) {
        self.timer.pause();
    }

    fn toggle(&mut self) {
        if self.is_running() {
            self.timer.pause();
        } else {
            self.timer.resume();
        }
    }
}
