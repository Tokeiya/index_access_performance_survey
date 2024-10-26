use chrono::{DateTime, TimeZone, Utc};

pub trait TimeStamper {
	type Tz:TimeZone;
	fn now()->DateTime<Self::Tz>;
}