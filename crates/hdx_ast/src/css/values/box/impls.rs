pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(MarginTop, 8);
		assert_size!(MarginRight, 8);
		assert_size!(MarginBottom, 8);
		assert_size!(MarginLeft, 8);
		assert_size!(Margin, 32);
		assert_size!(PaddingTop, 8);
		assert_size!(PaddingRight, 8);
		assert_size!(PaddingBottom, 8);
		assert_size!(PaddingLeft, 8);
		assert_size!(Padding, 32);
		// assert_size!(MarginTrim, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MarginLeft, "auto");
		assert_parse!(Margin, "1px 1px");
		assert_parse!(Margin, "1px 2px");
		assert_parse!(Margin, "1px 2px 3px 4px");
	}
}
