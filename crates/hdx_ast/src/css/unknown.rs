use hdx_lexer::Kind;
use hdx_parser::{Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::{
	component_values::{parse_component_values, ComponentValue},
	properties::CSSStyleProperty,
};
use crate::{Atom, Box, Spanned, Vec};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct UnknownAtRule<'a> {
	pub name: Atom,
	pub prelude: Box<'a, Option<Spanned<UnknownPrelude<'a>>>>,
	pub rules: Box<'a, Vec<'a, Spanned<UnknownRule<'a>>>>,
	pub properties: Box<'a, Vec<'a, Spanned<CSSStyleProperty<'a>>>>,
}

impl<'a> Parse<'a> for UnknownAtRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_at_rule(
			None,
			|parser: &mut Parser<'a>, name: Atom, prelude, rules, properties| {
				Ok(Self {
					name,
					prelude: parser.boxup(prelude),
					rules: parser.boxup(rules),
					properties: parser.boxup(properties),
				}
				.spanned(span.until(parser.cur().span)))
			},
		)
	}
}

impl<'a> WriteCss<'a> for UnknownAtRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str("@")?;
		sink.write_str(self.name.as_ref())?;
		if let Some(prelude) = &*self.prelude {
			prelude.write_css(sink)?;
			sink.write_trivia_char(' ')?;
		}
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.properties.iter().peekable();
		let mut rule_iter = self.rules.iter().peekable();
		while let Some(decl) = iter.next() {
			decl.write_css(sink)?;
			if iter.peek().is_none() && rule_iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in rule_iter {
			sink.write_newline()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		Ok(())
	}
}
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct UnknownRule<'a> {
	pub prelude: Box<'a, Option<Spanned<UnknownPrelude<'a>>>>,
	pub rules: Box<'a, Vec<'a, Spanned<UnknownRule<'a>>>>,
	pub properties: Box<'a, Vec<'a, Spanned<CSSStyleProperty<'a>>>>,
}

impl<'a> Parse<'a> for UnknownRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_qualified_rule(
			Some(Kind::Semicolon),
			true,
			|parser: &mut Parser<'a>, prelude, rules, properties| {
				Ok(Self {
					prelude: parser.boxup(prelude),
					rules: parser.boxup(rules),
					properties: parser.boxup(properties),
				}
				.spanned(span.until(parser.cur().span)))
			},
		)
	}
}

impl<'a> WriteCss<'a> for UnknownRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if let Some(prelude) = &*self.prelude {
			prelude.write_css(sink)?;
			sink.write_trivia_char(' ')?;
		}
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.properties.iter().peekable();
		let mut rule_iter = self.rules.iter().peekable();
		while let Some(decl) = iter.next() {
			decl.write_css(sink)?;
			if iter.peek().is_none() && rule_iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in rule_iter {
			sink.write_newline()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct UnknownPrelude<'a> {
	pub value: Box<'a, Vec<'a, Spanned<ComponentValue<'a>>>>,
}

impl<'a> Parse<'a> for UnknownPrelude<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		let value = parse_component_values(parser, Kind::Semicolon, false)?;
		Ok(Self { value: parser.boxup(value) }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for UnknownPrelude<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.value.iter().peekable();
		while let Some(value) = iter.next() {
			value.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(' ')?;
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
		use std::mem::size_of;
		assert_eq!(size_of::<UnknownAtRule>(), 32);
		assert_eq!(size_of::<UnknownRule>(), 24);
		assert_eq!(size_of::<UnknownPrelude>(), 8);
	}
}
