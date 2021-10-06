#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Telemetry {
    pub name: &'static str,
    pub duration: std::time::Duration,
}
