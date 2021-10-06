use super::Telemetry;
use std::sync::mpsc::Sender;
use std::time::Instant;

/// Timer used for benchmarking code.
/// When dropped it sends off the telemetry.
pub struct Timer {
    name: &'static str,
    start: Instant,
    telemetry_sender: Sender<Telemetry>,
}

impl Timer {
    pub(crate) fn start(name: &'static str, telemetry_sender: Sender<Telemetry>) -> Self {
        Self {
            name,
            start: Instant::now(),
            telemetry_sender,
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let duration = Instant::now() - self.start;

        match self.telemetry_sender.send(Telemetry {
            name: self.name,
            duration,
        }) {
            Ok(_) => {}
            Err(e) => {
                println!("Error dropping Benchy timer: {:?}", e)
            }
        }
    }
}
