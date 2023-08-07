use hdx_lexer::{Kind, Spanned};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{bitmask, Atomizable};

// https://drafts.csswg.org/css-box-4/#propdef-margin-trim
#[derive(Atomizable)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MarginTrim {
	None,
	Block,
	Inline,
	BlockStart,
	BlockEnd,
	InlineStart,
	InlineEnd,
}

impl<'a> Parse<'a> for MarginTrim {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		let mut value = Self::none();
		loop {
			if value.is_all() || !parser.at(Kind::Ident) {
				break;
			}
			if let Some(variant) = Self::from_atom(parser.cur_atom_lower().unwrap()) {
				if value.contains(variant) {
					break;
				}
				value |= variant;
				if variant == Self::None || variant == Self::Block || variant == Self::Inline {
					break;
				}
			}
			parser.advance();
		}
		if value.is_none() {
			Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?;
		}
		Ok(value.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for MarginTrim {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.contains(Self::BlockStart) {
			sink.write_str("block-start")?;
		}
		if self.contains(Self::BlockEnd) {
			if self.intersects(Self::BlockStart) {
				sink.write_char(' ')?;
			}
			sink.write_str("block-end")?;
		}
		if self.contains(Self::InlineStart) {
			if self.intersects(Self::BlockStart | Self::BlockEnd) {
				sink.write_char(' ')?;
			}
			sink.write_str("inline-start")?;
		}
		if self.contains(Self::InlineEnd) {
			if self.intersects(Self::BlockStart | Self::BlockEnd | Self::InlineStart) {
				sink.write_char(' ')?;
			}
			sink.write_str("inline-end")?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<MarginTrim>(), 1);
	}
}
