use std::ops::Deref;

use hdx_lexer::Kind;
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	css::{
		properties::CSSStyleProperty,
		rules::{page::CSSPageRule, CSSCharsetRule},
		selector::Selector,
		unknown::{UnknownAtRule, UnknownRule},
	},
	Atomizable, Box, Spanned, Vec,
};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct CSSStyleSheet<'a> {
	pub rules: Vec<'a, CSSRule<'a>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-stylesheet-contents
impl<'a> Parse<'a> for CSSStyleSheet<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		let mut rules = parser.new_vec();
		loop {
			match parser.cur().kind {
				Kind::Eof => break,
				Kind::Comment | Kind::Whitespace | Kind::Cdc | Kind::Cdo => parser.advance(),
				Kind::AtKeyword => {
					rules.push(match AtRuleId::from_atom(parser.cur_atom_lower().unwrap()) {
						Some(AtRuleId::Charset) => {
							let rule = CSSCharsetRule::parse(parser)?;
							CSSRule::Charset(parser.boxup(rule))
						}
						Some(AtRuleId::Page) => {
							let rule = CSSPageRule::parse(parser)?;
							CSSRule::Page(parser.boxup(rule))
						}
						None => {
							let rule = UnknownAtRule::parse(parser)?;
							parser.warn(diagnostics::UnknownRule(rule.span).into());
							CSSRule::UnknownAt(parser.boxup(rule))
						}
					});
				}
				_ => {
					// The spec talks of QualifiedRules but in the context of a Stylesheet
					// the only non-At Rule is a StyleRule, so parse that:
					let checkpoint = parser.checkpoint();
					match CSSStyleRule::parse(parser) {
						Ok(rule) => rules.push(CSSRule::Style(parser.boxup(rule))),
						Err(err) => {
							parser.rewind(checkpoint);
							parser.warn(err);
							let rule = UnknownRule::parse(parser)?;
							rules.push(CSSRule::Unknown(parser.boxup(rule)));
						}
					}
				}
			}
		}
		Ok(Self { rules }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for CSSStyleSheet<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		for rule in &self.rules {
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		Ok(())
	}
}

// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum CSSRule<'a> {
	Charset(Box<'a, Spanned<CSSCharsetRule>>),
	Page(Box<'a, Spanned<CSSPageRule<'a>>>),
	Style(Box<'a, Spanned<CSSStyleRule<'a>>>),
	UnknownAt(Box<'a, Spanned<UnknownAtRule<'a>>>),
	Unknown(Box<'a, Spanned<UnknownRule<'a>>>),
}

impl<'a> WriteCss<'a> for CSSRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Style(rule) => rule.write_css(sink),
			Self::Charset(rule) => rule.write_css(sink),
			Self::Page(rule) => rule.write_css(sink),
			Self::UnknownAt(rule) => rule.write_css(sink),
			Self::Unknown(rule) => rule.write_css(sink),
		}
	}
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum AtRuleId {
	Charset, // atom!("charset")
	Page,    // atom!("page")
}

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct CSSStyleRule<'a> {
	pub selectors: Box<'a, Spanned<SelectorSet<'a>>>,
	pub declarations: Box<'a, Vec<'a, Spanned<CSSStyleProperty<'a>>>>,
	pub rules: Box<'a, Vec<'a, Spanned<CSSStyleRule<'a>>>>,
}

impl<'a> Parse<'a> for CSSStyleRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_qualified_rule(
			None,
			false,
			|parser: &mut Parser<'a>,
			 selectors: Option<Spanned<SelectorSet<'a>>>,
			 rules: Vec<'a, Spanned<CSSStyleRule<'a>>>,
			 declarations: Vec<'a, Spanned<CSSStyleProperty<'a>>>| {
				if selectors.is_none() {
					Err(diagnostics::NoSelector(span, span.until(parser.cur().span)))?
				}
				Ok(Self {
					selectors: parser.boxup(selectors.unwrap()),
					declarations: parser.boxup(declarations),
					rules: parser.boxup(rules),
				}
				.spanned(span.until(parser.cur().span)))
			},
		)
	}
}

impl<'a> WriteCss<'a> for CSSStyleRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.selectors.write_css(sink)?;
		sink.write_trivia_char(' ')?;
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.declarations.deref().iter().peekable();
		while let Some(decl) = iter.next() {
			sink.write_indent()?;
			decl.write_css(sink)?;
			if iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		sink.dedent();
		sink.write_indent()?;
		sink.write_char('}')?;
		Ok(())
	}
}
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct SelectorSet<'a> {
	pub children: Vec<'a, Spanned<Selector<'a>>>,
}

impl<'a> Parse<'a> for SelectorSet<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		Ok(Self { children: parser.parse_comma_list_of::<Selector>()? }
			.spanned(span.until(parser.cur().span)))
	}
}
impl<'a> WriteCss<'a> for SelectorSet<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.children.iter().peekable();
		while let Some(selector) = iter.next() {
			selector.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_trivia_char(' ')?;
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
		assert_eq!(size_of::<CSSStyleSheet>(), 32);
		assert_eq!(size_of::<CSSRule>(), 16);
		assert_eq!(size_of::<AtRuleId>(), 1);
		assert_eq!(size_of::<CSSStyleRule>(), 24);
		assert_eq!(size_of::<SelectorSet>(), 32);
	}
}
