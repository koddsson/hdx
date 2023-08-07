#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::CSSFloat, Parseable, Writable};

#[derive(Parseable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum Zoom {
	#[default]
	Normal, // atom!("normal")
	Reset, // atom!("reset")
	#[parseable(kind = Number, from_token)]
	Number(CSSFloat),
	#[writable(suffix = "%")]
	#[parseable(kind = Percentage, from_token)]
	Percent(CSSFloat),
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Zoom>(), 8);
	}
}
