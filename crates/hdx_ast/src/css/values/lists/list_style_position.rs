use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-inline/#propdef-baseline-source
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum ListStylePosition {
	#[default]
	Outside, // atom!("outside")
	Inside, // atom!("inside")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ListStylePosition, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ListStylePosition, "inside");
		assert_parse!(ListStylePosition, "outside");
	}
}
