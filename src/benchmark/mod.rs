mod statistics;
mod telemetry;
pub mod timer;

use statistics::Stats;
use std::sync::Mutex;
pub(crate) use telemetry::Telemetry;
pub use timer::Timer;

lazy_static! {
    pub(crate) static ref STATISTICS: Mutex<Stats> = Mutex::new(Stats::new());
}

/// Creates a new timer.
pub(crate) fn timer(name: &'static str) -> Timer {
    Timer::start(name, STATISTICS.lock().unwrap().sender())
}

/// Saves the contents to the given file path.
pub(crate) fn save<'a>(file_path: &'a str) {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Error saving Benchy results: {:?}", e),
    };

    file.write_all(format!("{:#?}", STATISTICS.lock().unwrap().contents().borrow()).as_bytes())
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        let thread1 = std::thread::spawn(|| {
            timer("test1");
            std::thread::sleep(std::time::Duration::from_millis(400));
        });

        let thread2 = std::thread::spawn(|| {
            timer("test2");
            std::thread::sleep(std::time::Duration::from_millis(200));
        });

        thread1.join().unwrap();
        thread2.join().unwrap();

        assert_ne!(
            STATISTICS.lock().unwrap().contents().borrow().clone(),
            std::collections::HashMap::new()
        );

        //
        // Uncomment below to save a file.
        //
        // save("test.text");

        //
        // Uncomment below to see how it works
        //
        // assert_eq!(
        //     STATISTICS.lock().unwrap().contents().borrow().clone(),
        //     std::collections::HashMap::new()
        // );
    }
}
