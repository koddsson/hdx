use hdx_atom::Atom;
use hdx_lexer::{Kind, PairWise, Spanned, Token};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Box, Vec};

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ComponentValue<'a> {
	SimpleBlock(Spanned<SimpleBlock<'a>>),
	Function(Spanned<Function<'a>>),
	Token(Token),
}

// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
pub(crate) fn parse_component_values<'a>(
	parser: &mut Parser<'a>,
	stop_token: Kind,
	nested: bool,
) -> ParserResult<Vec<'a, Spanned<ComponentValue<'a>>>> {
	let mut values = parser.new_vec();
	loop {
		match parser.cur().kind {
			Kind::Eof => {
				return Ok(values);
			}
			Kind::RightCurly => {
				if nested {
					return Ok(values);
				}
				parser.advance();
			}
			c => {
				if c == stop_token {
					return Ok(values);
				}
				values.push(ComponentValue::parse(parser)?)
			}
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
impl<'a> Parse<'a> for ComponentValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::LeftCurly | Kind::LeftSquare | Kind::LeftParen => {
				Ok(Self::SimpleBlock(SimpleBlock::parse(parser)?)
					.spanned(span.until(parser.cur().span)))
			}
			Kind::Function => {
				Ok(Self::Function(Function::parse(parser)?).spanned(span.until(parser.cur().span)))
			}
			_ => {
				let token = parser.cur().clone();
				parser.advance();
				Ok(Self::Token(token).spanned(span))
			}
		}
	}
}

impl<'a> WriteCss<'a> for ComponentValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::SimpleBlock(b) => b.write_css(sink),
			Self::Function(f) => f.write_css(sink),
			Self::Token(t) => {
				match t.kind {
					Kind::Ident => sink.write_str(t.value.as_atom().unwrap().as_ref())?,
					Kind::AtKeyword => {
						sink.write_char('@')?;
						sink.write_str(t.value.as_atom().unwrap().as_ref())?;
					}
					Kind::Hash => {
						sink.write_char('#')?;
						if t.escaped {
							sink.write_char('\\')?;
						}
						sink.write_str(t.value.as_atom().unwrap().as_ref())?;
					}
					Kind::BadString | Kind::String => {
						if t.escaped {
							sink.write_char('\\')?;
						}
						sink.write_char('"')?;
						sink.write_str(t.value.as_atom().unwrap().as_ref())?;
						sink.write_char('"')?;
					}
					Kind::BadUrl | Kind::Url => {
						sink.write_str("url(")?;
						if t.escaped {
							sink.write_char('\\')?;
						}
						sink.write_str(t.value.as_atom().unwrap().as_ref())?;
						sink.write_char(')')?;
					}
					Kind::Delim => {
						sink.write_char(t.value.as_char().unwrap())?;
					}
					Kind::Number => sink.write_str(&format!("{}", t.value.as_f32().unwrap()))?,
					Kind::Percentage => {
						sink.write_str(&format!("{}", t.value.as_f32().unwrap()))?;
						sink.write_char('%')?;
					}
					Kind::Dimension => {
						sink.write_str(&format!("{}", t.value.as_f32().unwrap()))?;
						sink.write_str(t.value.as_atom().unwrap().as_ref())?;
					}
					Kind::Whitespace => sink.write_char(' ')?,
					Kind::Cdo => sink.write_str("<!--")?,
					Kind::Cdc => sink.write_str("-->")?,
					Kind::Colon => sink.write_char(':')?,
					Kind::Semicolon => sink.write_char(';')?,
					Kind::Comma => sink.write_char(',')?,
					Kind::LeftSquare => sink.write_char('[')?,
					Kind::RightSquare => sink.write_char(']')?,
					Kind::LeftParen => sink.write_char('(')?,
					Kind::RightParen => sink.write_char(')')?,
					Kind::LeftCurly => sink.write_char('{')?,
					Kind::RightCurly => sink.write_char('}')?,
					Kind::Undetermined => {}
					Kind::Comment => sink.write_trivia_str(t.value.as_atom().unwrap().as_ref())?,
					Kind::Function => {
						sink.write_str(t.value.as_atom().unwrap().as_ref())?;
						sink.write_char('(')?;
					}
					Kind::Eof => {}
				}
				Ok(())
			}
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct SimpleBlock<'a> {
	pub pairwise: PairWise,
	pub value: Box<'a, Vec<'a, Spanned<ComponentValue<'a>>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-simple-block
impl<'a> Parse<'a> for SimpleBlock<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		let pairwise = parser
			.cur()
			.to_pairwise()
			.ok_or_else(|| diagnostics::Unexpected(parser.cur().kind, span))?;
		parser.advance();
		let value = parse_component_values(parser, pairwise.end(), true)?;
		Ok(Self { value: parser.boxup(value), pairwise }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for SimpleBlock<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self.pairwise {
			PairWise::Square => sink.write_char('[')?,
			PairWise::Curly => sink.write_char('{')?,
			PairWise::Paren => sink.write_char('(')?,
		}
		for value in &*self.value {
			value.write_css(sink)?;
		}
		match self.pairwise {
			PairWise::Square => sink.write_char(']')?,
			PairWise::Curly => sink.write_char('}')?,
			PairWise::Paren => sink.write_char(')')?,
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Function<'a> {
	pub name: Atom,
	pub value: Box<'a, Vec<'a, Spanned<ComponentValue<'a>>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for Function<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		let name = parser.expect_function()?;
		let value = parse_component_values(parser, Kind::RightParen, false)?;
		Ok(Self { name, value: parser.boxup(value) }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for Function<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str(self.name.as_ref())?;
		sink.write_char('(')?;
		for value in &*self.value {
			value.write_css(sink)?;
		}
		sink.write_char(')')?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<ComponentValue>(), 32);
		assert_eq!(size_of::<SimpleBlock>(), 16);
		assert_eq!(size_of::<Function>(), 16);
	}
}
