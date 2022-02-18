use crate::counter::Counter;
use crate::input::{listen_for_inputs, Command};
use crate::notification::notify_default;
use crate::sound::play_bell;
use crate::terminal::{show_view, TermRawMode};
use anyhow::Result;
use std::{sync::mpsc::Receiver, time::Instant};

pub struct Timer {
    started: Instant,
    counter: u64,
    status: Status,
    stdout_raw: TermRawMode,
    input_receiver: Receiver<Command>,
}

enum Status {
    Running,
    Paused,
    Ended,
}

impl Timer {
    pub fn new(count: u64) -> Self {
        let stdout_raw = TermRawMode::new();
        let input_receiver = listen_for_inputs();

        Self {
            started: Instant::now(),
            counter: count,
            status: Status::Running,
            input_receiver,
            stdout_raw,
        }
    }

    fn alert(&self) -> Result<()> {
        notify_default("You break ended!", "Time for some work")?;
        play_bell()?;
        Ok(())
    }
}

impl Counter for Timer {
    fn has_ended(&self) -> bool {
        matches!(self.status, Status::Ended)
    }

    fn is_running(&self) -> bool {
        matches!(self.status, Status::Running)
    }

    fn is_paused(&self) -> bool {
        matches!(self.status, Status::Paused)
    }

    fn counter(&self) -> u64 {
        if self.is_running() {
            let elapsed = self.started.elapsed().as_secs();
            if self.counter > elapsed {
                self.counter - elapsed
            } else {
                0
            }
        } else {
            self.counter
        }
    }

    fn pause(&mut self) {
        if self.is_running() {
            self.counter = self.counter();
            self.status = Status::Paused;
        }
    }

    fn resume(&mut self) {
        if self.is_paused() {
            self.status = Status::Running;
            self.started = Instant::now();
        }
    }

    fn end_count(&mut self) {
        self.pause();
        self.status = Status::Ended;
    }

    fn update(&mut self) -> Result<()> {
        match self.input_receiver.try_recv() {
            Ok(Command::Quit) => {
                self.end_count();
                return Ok(());
            }

            Ok(Command::Pause) => {
                self.pause();
            }

            Ok(Command::Resume) => {
                self.resume();
            }

            Ok(Command::Toggle) | Ok(Command::Enter) => {
                self.toggle();
            }

            _ => (),
        }

        if self.counter() == 0 {
            self.end_count();
            self.alert()?;
            return Ok(());
        }

        let running = self.is_running();
        let counter = self.counter();
        show_view(&mut self.stdout_raw.stdout, counter, running)?;

        Ok(())
    }
}
