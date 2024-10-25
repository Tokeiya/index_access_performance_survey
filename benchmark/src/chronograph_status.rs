#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
	Reset,
	Stopped,
	Running,
}
