use std::{
    cell::RefCell,
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
};

use super::Telemetry;

/// The struct that contains all telemetry and statistics.
pub(crate) struct Stats {
    receiver: Receiver<Telemetry>,
    statistics: RefCell<HashMap<&'static str, Telemetry>>,
    sender: Sender<Telemetry>,
}

impl Stats {
    pub fn contents(&self) -> &RefCell<HashMap<&'static str, Telemetry>> {
        self.poll_contents();
        &self.statistics
    }

    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            receiver,
            sender,
            statistics: RefCell::new(HashMap::new()),
        }
    }

    pub fn sender(&self) -> Sender<Telemetry> {
        self.sender.clone()
    }

    fn poll_contents(&self) {
        let mut statistics = self.statistics.borrow_mut();

        for telemetry in self.receiver.try_iter() {
            match statistics.get_mut(telemetry.name) {
                Some(existing_telemetry) => {
                    existing_telemetry.duration += telemetry.duration;
                }
                None => {
                    //
                    statistics.insert(telemetry.name, telemetry);
                }
            }
        }
    }
}
