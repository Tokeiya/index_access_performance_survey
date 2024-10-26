use std::io::Write;

pub struct ResultWriter<T> {
	writer: T,
	delimiter: char,
}
