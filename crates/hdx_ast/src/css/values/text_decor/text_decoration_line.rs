use hdx_atom::atom;
use hdx_lexer::{Kind, Spanned};
use hdx_parser::{Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::bitmask;

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Default)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum TextDecorationLine {
	#[default]
	None = 0b0000,
	Underline = 0b0001,
	Overline = 0b0010,
	LineThrough = 0b0100,
	Blink = 0b1000,
}

impl<'a> Parse<'a> for TextDecorationLine {
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
			let ident = parser.cur_atom_lower().unwrap();
			match ident {
				atom!("underline") => {
					if value.contains(Self::Underline) {
						break;
					}
					value |= Self::Underline
				}
				atom!("overline") => {
					if value.contains(Self::Overline) {
						break;
					}
					value |= Self::Overline
				}
				atom!("line-through") => {
					if value.contains(Self::LineThrough) {
						break;
					}
					value |= Self::LineThrough
				}
				atom!("blink") => {
					if value.contains(Self::Blink) {
						break;
					}
					value |= Self::Blink
				}
				_ => break,
			}

			parser.advance()
		}
		Ok(value.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for TextDecorationLine {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.is_none() {
			sink.write_str("none")?;
		} else {
			if self.contains(Self::Underline) {
				sink.write_str("underline")?;
			}
			if self.contains(Self::Overline) {
				if self.intersects(Self::Overline) {
					sink.write_char(' ')?;
				}
				sink.write_str("overline")?;
			}
			if self.contains(Self::LineThrough) {
				if self.intersects(Self::Underline | Self::Overline) {
					sink.write_char(' ')?;
				}
				sink.write_str("line-through")?;
			}
			if self.contains(Self::Blink) {
				if self.intersects(Self::Underline | Self::Overline | Self::LineThrough) {
					sink.write_char(' ')?;
				}
				sink.write_str("blink")?;
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
		assert_eq!(::std::mem::size_of::<TextDecorationLine>(), 1);
	}
}
