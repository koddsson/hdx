use hdx_lexer::{Kind, Spanned};
use hdx_parser::{Parse, Parser, Result as ParserResult};

pub mod charset;
pub mod page;

pub use charset::*;
pub use page::*;

pub struct NoPreludeAllowed;
impl<'a> Parse<'a> for NoPreludeAllowed {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect_without_advance(Kind::LeftCurly)?;
		Ok(Self {}.spanned(span.until(parser.cur().span)))
	}
}
