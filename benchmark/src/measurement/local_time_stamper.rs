use crate::measurement::time_stamper::TimeStamper;
use chrono::{DateTime, Local};

pub struct LocalTimeStamper;
impl TimeStamper for LocalTimeStamper {
	type Tz = Local;
	fn now() -> DateTime<Self::Tz> {
		Local::now()
	}
}
