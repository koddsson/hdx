#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parseable, Writable};

// https://drafts.csswg.org/css-break-4/#propdef-margin-break
#[derive(Parseable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MarginBreak {
	#[default]
	Auto, // atom!("auto")
	Keep,    // atom!("keep")
	Discard, // atom!("discard")
}
