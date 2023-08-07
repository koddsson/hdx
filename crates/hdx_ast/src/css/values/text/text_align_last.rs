#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parseable, Writable};

// https://drafts.csswg.org/css-text-4/#propdef-text-align-last
#[derive(Parseable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextAlignLast {
	#[default]
	Auto, // atom!("auto")
	Start,       // atom!("start")
	End,         // atom!("end")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Center,      // atom!("center")
	Justify,     // atom!("justify")
	MatchParent, // atom!("match-parent")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<TextAlignLast>(), 1);
	}
}
