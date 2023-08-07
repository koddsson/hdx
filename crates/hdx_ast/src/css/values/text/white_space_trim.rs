use hdx_atom::atom;
use hdx_lexer::{Kind, Spanned};
use hdx_parser::{Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{bitmask, Atomizable};

// https://drafts.csswg.org/css-text-4/#propdef-white-space-trim
#[derive(Default, Atomizable)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum WhiteSpaceTrim {
	#[default]
	None = 0b0000, // atom!("none")
	DiscardBefore = 0b0001, // atom!("discard-before")
	DiscardAfter = 0b0010,  // atom!("discard-after")
	DiscardInner = 0b0100,  // atom!("discard-inner")
}

impl<'a> Parse<'a> for WhiteSpaceTrim {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		if parser.at(Kind::Ident) && parser.cur().as_atom_lower().unwrap() == atom!("none") {
			parser.advance();
			return Ok(Self::None.spanned(span.until(parser.cur().span)));
		}
		let mut value = Self::none();
		loop {
			if value.is_all() || !parser.at(Kind::Ident) {
				break;
			}
			if let Some(variant) = Self::from_atom(parser.cur_atom_lower().unwrap()) {
				if value.contains(variant) {
					break;
				}
				value |= variant
			}
			parser.advance();
		}
		Ok(value.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for WhiteSpaceTrim {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.is_none() {
			sink.write_str("none")?;
		} else {
			if self.contains(Self::DiscardBefore) {
				sink.write_str("discard-before")?;
			}
			if self.contains(Self::DiscardAfter) {
				if self.intersects(Self::DiscardBefore) {
					sink.write_char(' ')?;
				}
				sink.write_str("discard-after")?;
			}
			if self.contains(Self::DiscardInner) {
				if self.intersects(Self::DiscardBefore | Self::DiscardAfter) {
					sink.write_char(' ')?;
				}
				sink.write_str("discard-inner")?;
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<WhiteSpaceTrim>(), 1);
	}
}
