use hdx_atom::{atom, Atom};
use hdx_lexer::{Kind, Spanned};
use hdx_parser::{Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// TODO: maybe make this an enum? Can use:
// https://www.iana.org/assignments/character-sets/character-sets.xhtml
pub struct CSSCharsetRule {
	// Common charsets
	// atom!("UTF-8")
	// atom!("utf-8")
	pub encoding: Atom,
}

impl<'a> Parse<'a> for CSSCharsetRule {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect_at_keyword_of(atom!("charset"))?;
		let encoding = parser.expect_string()?;
		parser.expect(Kind::Semicolon)?;
		Ok(Self { encoding }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for CSSCharsetRule {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str("@charset \"")?;
		sink.write_str(self.encoding.as_ref())?;
		sink.write_str("\";")?;
		Ok(())
	}
}
