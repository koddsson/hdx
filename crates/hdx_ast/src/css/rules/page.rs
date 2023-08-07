use hdx_lexer::Kind;
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::NoPreludeAllowed;
use crate::{
	atom, css::properties::CSSStyleProperty, Atom, Atomizable, Box, Spanned, Specificity,
	ToSpecificity, Vec,
};

// https://drafts.csswg.org/cssom-1/#csspagerule
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct CSSPageRule<'a> {
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub selectors: Box<'a, Spanned<PageSelectorList<'a>>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Box<'a, Vec<'a, Spanned<CSSStyleProperty<'a>>>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Box<'a, Vec<'a, Spanned<CSSMarginRule<'a>>>>,
}

impl<'a> Parse<'a> for CSSPageRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_at_rule(
			Some(atom!("page")),
			|parser: &mut Parser<'a>,
			 _name: Atom,
			 selectors: Option<Spanned<PageSelectorList<'a>>>,
			 rules: Vec<'a, Spanned<CSSMarginRule<'a>>>,
			 properties: Vec<'a, Spanned<CSSStyleProperty<'a>>>| {
				Ok(Self {
					selectors: parser.boxup(selectors.unwrap_or_else(|| {
						Spanned::dummy(PageSelectorList { children: parser.new_vec() })
					})),
					properties: parser.boxup(properties),
					rules: parser.boxup(rules),
				}
				.spanned(span.until(parser.cur().span)))
			},
		)
	}
}

impl<'a> WriteCss<'a> for CSSPageRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str("@page")?;
		if self.selectors.node.children.len() > 0 {
			sink.write_char(' ')?;
		}
		self.selectors.write_css(sink)?;
		if self.selectors.node.children.len() > 0 {
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
		sink.dedent();
		sink.write_indent()?;
		sink.write_char('}')?;
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageSelectorList<'a> {
	pub children: Vec<'a, Spanned<PageSelector<'a>>>,
}

impl<'a> Parse<'a> for PageSelectorList<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		let ok = Ok(Self { children: parser.parse_comma_list_of::<PageSelector>()? }
			.spanned(span.until(parser.cur().span)));
		ok
	}
}

impl<'a> WriteCss<'a> for PageSelectorList<'a> {
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

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageSelector<'a> {
	pub page_type: Option<Atom>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub pseudos: Vec<'a, Spanned<PagePseudoClass>>,
}

impl<'a> Parse<'a> for PageSelector<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		let mut page_type = None;
		let mut pseudos = parser.new_vec();
		if parser.at(Kind::Ident) {
			page_type = Some(parser.expect_ident()?);
		} else {
			parser.expect_without_advance(Kind::Colon)?;
		}
		if parser.at(Kind::Colon) {
			loop {
				pseudos.push(PagePseudoClass::parse(parser)?);
				if !parser.at(Kind::Colon) {
					break;
				}
			}
		}
		Ok(Self { page_type, pseudos }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for PageSelector<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if let Some(page_type) = &self.page_type {
			sink.write_str(page_type.as_ref())?;
		}
		for pseudo in self.pseudos.iter() {
			sink.write_char(':')?;
			sink.write_str(pseudo.to_atom().as_ref())?;
		}
		Ok(())
	}
}

impl<'a> PageSelector<'a> {
	pub fn selector(&self) -> &str {
		todo!();
		// format!("{}{}", self.page_type.unwrap_or("").to_owned(), self.pseudos.into_iter().fold("", |p| p.as_str())join("")).as_str()
	}

	pub fn specificity(&self) -> Specificity {
		let mut spec = Specificity(self.page_type.is_some() as u8, 0, 0);
		for pseudo in &self.pseudos {
			spec += pseudo.specificity();
		}
		spec
	}
}

#[derive(Atomizable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "kebab-case"))]
pub enum PagePseudoClass {
	Left,
	Right,
	First,
	Blank,
}

impl<'a> Parse<'a> for PagePseudoClass {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect(Kind::Colon)?;
		let name = parser.expect_ident()?;
		match Self::from_atom(name.clone()) {
			Some(v) => Ok(v.spanned(span.until(parser.cur().span))),
			_ => Err(diagnostics::UnexpectedPseudo(name, span).into()),
		}
	}
}

impl ToSpecificity for PagePseudoClass {
	fn specificity(&self) -> Specificity {
		match self {
			Self::Blank => Specificity(0, 1, 0),
			Self::First => Specificity(0, 1, 0),
			Self::Left => Specificity(0, 0, 1),
			Self::Right => Specificity(0, 0, 1),
		}
	}
}

// https://drafts.csswg.org/cssom-1/#cssmarginrule
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct CSSMarginRule<'a> {
	pub name: PageMarginBox,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, Spanned<CSSStyleProperty<'a>>>,
}

impl<'a> Parse<'a> for CSSMarginRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		parser.parse_at_rule(
			None,
			|parser: &mut Parser<'a>,
			 _name: Atom,
			 _prelude: Option<Spanned<NoPreludeAllowed>>,
			 _rules: Vec<'a, Spanned<CSSMarginRule<'a>>>,
			 properties: Vec<'a, Spanned<CSSStyleProperty<'a>>>| {
				Ok(Self { name: PageMarginBox::TopLeft, properties }
					.spanned(span.until(parser.cur().span)))
			},
		)
	}
}

impl<'a> WriteCss<'a> for CSSMarginRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('@')?;
		sink.write_str(self.name.to_atom().as_ref())?;
		sink.write_trivia_char(' ')?;
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.properties.iter().peekable();
		while let Some(decl) = iter.next() {
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

#[derive(Atomizable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "kebab-case"))]
pub enum PageMarginBox {
	TopLeftCorner,     // atom!("top-left-corner")
	TopLeft,           // atom!("top-left")
	TopCenter,         // atom!("top-center")
	TopRight,          // atom!("top-right")
	TopRightCorner,    // atom!("top-right-corner")
	RightTop,          // atom!("right-top")
	RightMiddle,       // atom!("right-middle")
	RightBottom,       // atom!("right-bottom")
	BottomRightCorner, // atom!("bottom-right-corner")
	BottomRight,       // atom!("bottom-right")
	BottomCenter,      // atom!("bottom-center")
	BottomLeft,        // atom!("bottom-left")
	BottomLeftCorner,  // atom!("bottom-left-corner")
	LeftBottom,        // atom!("left-bottom")
	LeftMiddle,        // atom!("left-middle")
	LeftTop,           // atom!("left-top")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<CSSPageRule>(), 24);
		assert_eq!(size_of::<CSSMarginRule>(), 40);
		assert_eq!(size_of::<PagePseudoClass>(), 1);
		assert_eq!(size_of::<PageMarginBox>(), 1);
		assert_eq!(size_of::<PagePseudoClass>(), 1);
	}

	#[test]
	fn test_specificity() {
		assert_eq!(PagePseudoClass::Left.specificity(), Specificity(0, 0, 1));
		assert_eq!(PagePseudoClass::Right.specificity(), Specificity(0, 0, 1));
		assert_eq!(PagePseudoClass::First.specificity(), Specificity(0, 1, 0));
		assert_eq!(PagePseudoClass::Blank.specificity(), Specificity(0, 1, 0));
	}
}
