use std::hash::{Hash, Hasher};

use hdx_atom::atom;
use hdx_lexer::{Kind, Spanned};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

// https://drafts.csswg.org/css-size-adjust-1/#propdef-text-size-adjust
#[derive(Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum TextSizeAdjust {
	None,
	#[default]
	Auto,
	Percentage(f32),
}

impl Hash for TextSizeAdjust {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			TextSizeAdjust::None => state.write_u8(0),
			TextSizeAdjust::Auto => state.write_u8(1),
			TextSizeAdjust::Percentage(v) => {
				state.write_u8(2);
				v.to_bits().hash(state);
			}
		}
	}
}

impl<'a> WriteCss<'a> for TextSizeAdjust {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::None => sink.write_str("none"),
			Self::Auto => sink.write_str("auto"),
			Self::Percentage(n) => sink.write_str(&format!("{}%", n)),
		}
	}
}

impl<'a> Parse<'a> for TextSizeAdjust {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.cur_atom().unwrap();
				match ident.to_ascii_lowercase() {
					atom!("none") => Ok(Self::None.spanned(span)),
					atom!("auto") => Ok(Self::Auto.spanned(span)),
					_ => Err(diagnostics::UnexpectedIdent(
						parser.cur_atom().unwrap(),
						parser.cur().span,
					))?,
				}
			}
			Kind::Percentage => {
				let value = parser.expect_number()?;
				Ok(Self::Percentage(value).spanned(span))
			}
			_ => Err(diagnostics::Unexpected(parser.cur().kind, parser.cur().span))?,
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<TextSizeAdjust>(), 8);
	}
}
