#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parseable, Writable};

#[derive(Parseable, Writable, Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum PointerEvents {
	Auto, // atom!("auto")
	None, // atom!("none")
}
