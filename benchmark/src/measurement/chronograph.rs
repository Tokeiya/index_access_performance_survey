#[cfg(test)]
pub(super) mod mock_instant {
	use std::ops::DerefMut;
	use std::sync::{LazyLock, Mutex};
	use std::time::{Duration, Instant};

	pub static MOCK_INSTANT: LazyLock<Mutex<Instant>> =
		LazyLock::new(|| Mutex::new(Instant::now()));
	pub static MOCK_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::from_secs(1));

	pub fn now() -> Instant {
		let mut guard = MOCK_INSTANT.lock().unwrap();
		*(guard.deref_mut()) += *MOCK_DURATION;
		dbg!(*guard);
		*guard
	}
}

pub(super) mod instant {
	use std::time::Instant;

	#[allow(dead_code)]
	pub fn now() -> Instant {
		Instant::now()
	}
}

use crate::measurement::chronograph_status::Status;
#[cfg(not(test))]
use instant::now;
#[cfg(test)]
use mock_instant::now;
use std::time::{Duration, Instant};

pub struct Chronograph {
	pivot: Option<Instant>,
	accumulator: Duration,
}

impl Default for Chronograph {
	fn default() -> Self {
		Self {
			pivot: None,
			accumulator: Duration::from_secs(0),
		}
	}
}

impl Chronograph {
	pub fn start(&mut self) {
		if self.pivot.is_none() {
			self.pivot = Some(now());
		}
	}

	pub fn stop(&mut self) {
		let current = now();

		if self.pivot.is_some() {
			let diff = current - self.pivot.unwrap();
			self.accumulator += diff;
			self.pivot = None;
		}
	}

	pub fn reset(&mut self) {
		self.accumulator = Duration::default();
		self.pivot = None;
	}

	pub fn restart(&mut self) {
		self.pivot = Some(now());
		self.accumulator = Duration::default();
	}

	pub fn status(&self) -> Status {
		match (self.pivot.is_none(), self.accumulator.is_zero()) {
			(true, true) => Status::Reset,
			(true, false) => Status::Stopped,
			_ => Status::Running,
		}
	}

	pub fn elapsed(&self) -> Duration {
		let current = now();

		if let Some(piv) = self.pivot {
			self.accumulator + (current - piv)
		} else {
			self.accumulator
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serial_test::serial;
	use std::time::Duration;

	#[serial]
	#[test]
	fn default() {
		let fixture = Chronograph::default();
		assert_eq!(fixture.status(), Status::Reset);
		assert!(fixture.accumulator.is_zero());
		assert!(fixture.elapsed().is_zero());
		assert!(fixture.pivot.is_none());
	}

	#[serial]
	#[test]
	fn change_state() {
		let mut fixture = Chronograph::default();
		assert_eq!(fixture.status(), Status::Reset);

		fixture.start();
		assert_eq!(fixture.status(), Status::Running);
		let recent = fixture.pivot.unwrap();

		fixture.start();
		assert_eq!(fixture.status(), Status::Running);
		assert_eq!(fixture.pivot.unwrap(), recent);

		fixture.stop();
		assert_eq!(fixture.status(), Status::Stopped);
		assert!(fixture.pivot.is_none());
		assert_eq!(fixture.elapsed(), Duration::from_secs(1));

		fixture.reset();
		assert_eq!(fixture.status(), Status::Reset);
		assert!(fixture.pivot.is_none());
	}

	#[serial]
	#[test]
	fn restart() {
		let mut fixture = Chronograph::default();
		fixture.start();
		let recent = fixture.pivot.unwrap();

		fixture.restart();
		let current = fixture.pivot.unwrap();
		assert_eq!(current - recent, Duration::from_secs(1));
	}

	#[serial]
	#[test]
	fn reset() {
		let mut fixture = Chronograph::default();
		fixture.start();
		assert!(fixture.pivot.is_some());

		fixture.stop();
		assert_eq!(fixture.accumulator, Duration::from_secs(1));

		fixture.start();
		assert!(fixture.pivot.is_some());

		fixture.reset();
		assert!(fixture.pivot.is_none());
		assert_eq!(fixture.accumulator, Duration::from_secs(0));

		fixture.start();
		assert!(fixture.pivot.is_some());

		fixture.stop();
		assert_eq!(fixture.accumulator, Duration::from_secs(1));

		fixture.reset();
		assert!(fixture.pivot.is_none());
		assert_eq!(fixture.accumulator, Duration::from_secs(0));
	}

	#[test]
	fn elapsed() {
		let mut fixture = Chronograph::default();
		fixture.start();
		fixture.stop();

		fixture.start();
		assert_eq!(fixture.elapsed(), Duration::from_secs(2));
	}
}
