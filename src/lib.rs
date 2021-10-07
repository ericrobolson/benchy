#![cfg_attr(not(feature = "benchmark"), no_std)]

#[cfg(feature = "benchmark")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "benchmark")]
mod benchmark;

/// Profiling interface for code. Typically used like the following:
///
/// ```
/// use benchy::Benchy;
///
/// // Scope A
/// {
///     Benchy::time("test1");
///
///     // do some work
///
///     // Scope A exits, so timer was dropped and logs duration.
/// }
///
/// // Call save to save results to disk.
/// Benchy::save("someRandomFile.txt");
/// ```
pub struct Benchy {}

impl Benchy {
    /// Creates a timer to benchmark a scoped block. When the scope exits, the timer is dropped.
    /// When the timer is dropped, the time it existed is logged.
    #[cfg(feature = "benchmark")]
    pub fn time(name: &'static str) -> benchmark::Timer {
        benchmark::timer(name)
    }

    /// Empty function for when the benchmark feature is not enabled.
    #[cfg(not(feature = "benchmark"))]
    pub fn time(_name: &'static str) {}

    /// Saves the benchmark results to the given file.
    #[allow(unused_variables)]
    pub fn save<'a>(file_path: &'a str) {
        #[cfg(feature = "benchmark")]
        {
            benchmark::save(file_path);
        }
    }
}
