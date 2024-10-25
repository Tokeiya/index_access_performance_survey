#[cfg(test)]
mod mock_instant {
	use std::sync::{LazyLock, Mutex};
	use std::time::{Duration, Instant};

	pub(super) static MOCK_INSTANT: LazyLock<Mutex<Instant>> =
		LazyLock::new(|| Mutex::new(Instant::now()));
	pub(super) static MOCK_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::from_secs(1));

	pub(super) fn now() -> Instant {
		todo!()
	}
}

mod instant {
	use std::time::Instant;
	pub(super) fn now() -> Instant {
		Instant::now()
	}
}

#[cfg(not(test))]
use instant::now;
#[cfg(test)]
use mock_instant::now;
use std::time::{Duration, Instant};

pub enum Status {
	Reset,
	Stop,
	Running,
}

pub struct Chronograph {
	start: Option<Instant>,
	accumulator: Duration,
}

impl Chronograph {
	pub fn new() -> Self {
		todo!()
	}

	pub fn start(&mut self) {
		todo!()
	}

	pub fn stop(&mut self) {
		todo!()
	}

	pub fn reset(&mut self) {
		todo!()
	}

	pub fn restart(&mut self) {
		todo!()
	}

	pub fn status(&self) -> Status {
		todo!()
	}

	pub fn elapsed(&self) -> Duration {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::mock_instant::{MOCK_DURATION, MOCK_INSTANT};
	use super::*;
	use std::time::Duration;

	#[test]
	fn new() {
		todo!();
	}

	#[test]
	fn change_state() {
		todo!();
	}

	#[test]
	fn accumulate() {
		todo!();
	}

	#[test]
	fn reset() {
		todo!();
	}

	#[test]
	fn restart() {
		todo!();
	}
}
