use hdx_atom::atom;
use hdx_lexer::{Kind, Spanned};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

// https://drafts.csswg.org/css-page-floats-3/#propdef-float-defer
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FloatDefer {
	#[default]
	Last,
	None,
	Integer(i32),
}

impl<'a> WriteCss<'a> for FloatDefer {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Last => sink.write_str("normal"),
			Self::None => sink.write_str("reset"),
			Self::Integer(n) => sink.write_str(&n.to_string()),
		}
	}
}

impl<'a> Parse<'a> for FloatDefer {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.cur_atom().unwrap();
				match ident.to_ascii_lowercase() {
					atom!("last") => Ok(Self::Last.spanned(span)),
					atom!("none") => Ok(Self::None.spanned(span)),
					_ => Err(diagnostics::UnexpectedIdent(
						parser.cur_atom().unwrap(),
						parser.cur().span,
					))?,
				}
			}
			Kind::Number => {
				let value = parser.expect_int()?;
				Ok(Self::Integer(value).spanned(span))
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
		assert_eq!(::std::mem::size_of::<FloatDefer>(), 8);
	}
}
