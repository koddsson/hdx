use hdx_atom::{atom, Atom};
use hdx_lexer::{Kind, Spanned};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::{parse_wq_name, NSPrefix};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Attribute {
	pub ns_prefix: NSPrefix,
	pub name: Atom,
	pub value: Atom,
	pub matcher: AttributeMatch,
	pub modifier: AttributeModifier,
}

impl<'a> Parse<'a> for Attribute {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect(Kind::LeftSquare)?;
		let (ns_prefix, name) = parse_wq_name(parser)?;
		let mut matcher = AttributeMatch::Any;
		let mut modifier = AttributeModifier::None;
		let mut value = atom!("");
		match parser.cur().kind {
			Kind::RightSquare => {
				parser.next_token();
				return Ok(Self { ns_prefix, name, value, modifier, matcher }
					.spanned(span.until(parser.cur().span)));
			}
			Kind::Delim => {
				let delim_span = parser.cur().span;
				let ch = parser.cur().value.as_char().unwrap();
				parser.next_token();
				if matcher != AttributeMatch::Any {
					Err(diagnostics::UnexpectedDelim(ch, delim_span))?;
				}
				matcher = match ch {
					'=' => AttributeMatch::Exact,
					'~' => AttributeMatch::SpaceList,
					'|' => AttributeMatch::LangPrefix,
					'^' => AttributeMatch::Prefix,
					'$' => AttributeMatch::Suffix,
					'*' => AttributeMatch::Contains,
					_ => Err(diagnostics::UnexpectedDelim(ch, delim_span))?,
				};
				if ch != '=' {
					let ch = parser.expect_delim()?;
					if ch != '=' {
						Err(diagnostics::UnexpectedDelim(ch, delim_span))?;
					}
				}
			}
			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
		}
		match parser.cur().kind {
			Kind::Ident | Kind::String => {
				value = parser.cur().as_atom().unwrap();
				parser.advance();
			}
			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
		}
		match parser.cur().kind {
			Kind::RightSquare => {
				parser.next_token();
				Ok(Self { ns_prefix, name, value, modifier, matcher }
					.spanned(span.until(parser.cur().span)))
			}
			Kind::Ident => {
				let ident_span = parser.cur().span;
				modifier = match parser.expect_ident()? {
					atom!("i") => AttributeModifier::Insensitive,
					atom!("s") => AttributeModifier::Sensitive,
					a => Err(diagnostics::UnexpectedIdent(a, ident_span))?,
				};
				parser.expect(Kind::RightSquare)?;
				Ok(Self { ns_prefix, name, value, modifier, matcher }
					.spanned(span.until(parser.cur().span)))
			}
			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
		}
	}
}

impl<'a> WriteCss<'a> for Attribute {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('[')?;
		match &self.ns_prefix {
			NSPrefix::None => {}
			NSPrefix::Named(ns) => {
				sink.write_str(ns.as_ref())?;
				sink.write_char('|')?;
			}
			NSPrefix::Wildcard => {
				sink.write_char('*')?;
				sink.write_char('|')?;
			}
		}
		sink.write_str(self.name.as_ref())?;
		match &self.matcher {
			AttributeMatch::Any => {}
			AttributeMatch::Exact => {
				sink.write_char('=')?;
			}
			AttributeMatch::SpaceList => {
				sink.write_char('~')?;
				sink.write_char('=')?;
			}
			AttributeMatch::LangPrefix => {
				sink.write_char('|')?;
				sink.write_char('=')?;
			}
			AttributeMatch::Prefix => {
				sink.write_char('^')?;
				sink.write_char('=')?;
			}
			AttributeMatch::Suffix => {
				sink.write_char('$')?;
				sink.write_char('=')?;
			}
			AttributeMatch::Contains => {
				sink.write_char('*')?;
				sink.write_char('=')?;
			}
		}
		if &self.matcher != &AttributeMatch::Any {
			sink.write_char('"')?;
			sink.write_str(self.value.as_ref())?;
			sink.write_char('"')?;
		}

		sink.write_char(']')?;
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum AttributeMatch {
	Any,        // [attr]
	Exact,      // [attr=val]
	SpaceList,  // [attr~=val]
	LangPrefix, // [attr|=val]
	Prefix,     // [attr^=val]
	Suffix,     // [attr$=val]
	Contains,   // [attr*=val]
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum AttributeModifier {
	None,
	Sensitive,
	Insensitive,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Attribute>(), 40);
		assert_eq!(::std::mem::size_of::<AttributeMatch>(), 1);
		assert_eq!(::std::mem::size_of::<AttributeMatch>(), 1);
	}
}
