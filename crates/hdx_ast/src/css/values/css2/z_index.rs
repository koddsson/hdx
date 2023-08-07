use hdx_atom::atom;
use hdx_lexer::{Kind, Spanned};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum ZIndex {
	#[default]
	Auto,
	Integer(i32),
}

impl<'a> WriteCss<'a> for ZIndex {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Auto => sink.write_str("auto"),
			Self::Integer(i) => sink.write_str(&i.to_string()),
		}
	}
}

impl<'a> Parse<'a> for ZIndex {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				parser.expect_ident_of(atom!("auto"))?;
				Ok(Self::Auto.spanned(span))
			}
			Kind::Number => {
				let i = parser.expect_int()?;
				Ok(Self::Integer(i).spanned(span))
			}
			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<ZIndex>(), 8);
	}
}
