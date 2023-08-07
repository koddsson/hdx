use hdx_atom::Atom;
use hdx_lexer::{Kind, Span, Spanned};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Box, Vec};

mod attribute;
mod pseudo_class;

use attribute::Attribute;
use pseudo_class::PseudoClass;

// This encapsulates both `simple-selector` and `compound-selector`.
// As `simple-selector` is a `compound-selector` but with only one `Component`.
// Having `Selector` be both ` simple-selector` and `compound-selector` makes parsing and visiting
// more practical.
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Selector<'a> {
	pub components: Box<'a, Vec<'a, Spanned<Component<'a>>>>,
}

impl<'a> Parse<'a> for Selector<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		let mut components: Vec<'a, Spanned<Component>> = parser.new_vec();
		loop {
			match parser.cur().kind {
				Kind::Eof | Kind::Semicolon | Kind::Comma | Kind::LeftCurly => {
					break;
				}
				Kind::Whitespace => {
					if matches!(
						parser.peek().kind,
						Kind::Eof | Kind::Semicolon | Kind::Comma | Kind::LeftCurly
					) {
						break;
					}
				}
				_ => {}
			}
			let component = Component::parse(parser)?;
			if let Some(Spanned { node, span: component_span }) = components.last() {
				match (node, &component.node) {
					// A selector like `a /**/ b` would parse as // <Type>, <Descendant>,
					// <Descendant>, <Type>. The CSS selector grammar implicitly swallows adjacent
					// descendant combinators as whitespace, but due to simplifying AST nodes in our
					// parser, it means we must explicitly check for, and elide adjacent descendant
					// combinators. Adjacent Descendant Combinator Elision is the name of my metal
					// band, btw.
					(Component::Combinator(_), Component::Combinator(Combinator::Descendant))
					| (Component::Combinator(Combinator::Descendant), Component::Combinator(_)) => {
						continue;
					}
					// Combinators cannot be next to eachother.
					(Component::Combinator(_), Component::Combinator(_)) => {
						Err(diagnostics::AdjacentSelectorCombinators(
							*component_span,
							Span::new(span.start, component_span.start),
						))?
					}
					// Types cannot be next to eachother.
					(Component::Type(_), Component::Type(_)) => {
						Err(diagnostics::AdjacentSelectorTypes(
							*component_span,
							Span::new(span.start, component_span.start),
						))?
					}
					_ => {}
				}
			}
			components.push(component);
		}
		Ok(Self { components: parser.boxup(components) }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for Selector<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		for component in &(*self.components) {
			component.write_css(sink)?;
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ForgivingSelector<'a> {
	pub components: Box<'a, Vec<'a, Spanned<Component<'a>>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct RelativeSelector<'a> {
	pub components: Box<'a, Vec<'a, Spanned<Component<'a>>>>,
}

// This encapsulates all `simple-selector` subtypes (e.g. `wq-name`,
// `id-selector`) into one enum, as it makes parsing and visiting much more practical.
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum Component<'a> {
	Id(Atom),
	Class(Atom),
	Type(Atom),
	Wildcard,
	Combinator(Combinator),
	Attribute(Box<'a, Spanned<Attribute>>),
	PseudoClass(PseudoClass),
	PseudoElement(PseudoElement),
	LegacyPseudoElement(LegacyPseudoElement),
	PseudoFunction(PseudoFunction<'a>),
	NSPrefixedType(Box<'a, (NSPrefix, Atom)>),
	NSPrefixedWildcard(NSPrefix),
}

impl<'a> Parse<'a> for Component<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Whitespace => {
				parser.next_token();
				Ok(Self::Combinator(Combinator::Descendant).spanned(span.until(parser.cur().span)))
			}
			Kind::Ident => {
				let name = parser.cur().as_atom_lower().unwrap();
				parser.next_token();
				Ok(Self::Type(name).spanned(span))
			}
			Kind::Colon => {
				parser.next_token();
				match parser.cur().kind {
					Kind::Colon => {
						parser.next_token();
						parser.expect_without_advance(Kind::Ident)?;
						let ident = parser.cur().as_atom().unwrap();
						if let Some(selector) = PseudoElement::from_atom(ident) {
							parser.next_token();
							Ok(Self::PseudoElement(selector).spanned(span.until(parser.cur().span)))
						} else {
							Err(diagnostics::Unimplemented(parser.cur().span))?
						}
					}
					Kind::Ident => {
						parser.expect_without_advance(Kind::Ident)?;
						let ident = parser.cur().as_atom().unwrap();
						if let Some(selector) = PseudoClass::from_atom(ident.clone()) {
							parser.next_token();
							Ok(Self::PseudoClass(selector).spanned(span.until(parser.cur().span)))
						} else if let Some(e) = LegacyPseudoElement::from_atom(ident.clone()) {
							parser.next_token();
							Ok(Self::LegacyPseudoElement(e).spanned(span.until(parser.cur().span)))
						} else {
							Err(diagnostics::UnexpectedIdent(ident, parser.cur().span))?
						}
					}
					_ => Err(diagnostics::Unimplemented(parser.cur().span))?,
				}
			}
			Kind::Hash => {
				let name = parser.cur().as_atom().unwrap();
				parser.next_token();
				Ok(Self::Id(name).spanned(span.until(parser.cur().span)))
			}
			Kind::Delim => match parser.cur().value.as_char() {
				Some('.') => {
					let next_token = parser.peek_including_trivia();
					match next_token.kind {
						Kind::Ident => {
							parser.next_token();
							let ident = parser.cur().as_atom().unwrap();
							parser.next_token();
							Ok(Self::Class(ident).spanned(span.until(parser.cur().span)))
						}
						_ => Err(diagnostics::Unimplemented(parser.cur().span))?,
					}
				}
				Some('*') => {
					let next_token = parser.peek_including_trivia();
					match next_token.kind {
						Kind::Delim if next_token.value.as_char().unwrap() == '|' => {
							let (prefix, atom) = parse_wq_name(parser)?;
							Ok(Self::NSPrefixedType(parser.boxup((prefix, atom)))
								.spanned(span.until(parser.cur().span)))
						}
						_ => {
							parser.next_token();
							Ok(Self::Wildcard.spanned(span.until(parser.cur().span)))
						}
					}
				}
				_ => Err(diagnostics::Unimplemented(parser.cur().span))?,
			},
			Kind::LeftSquare => {
				let attr = Attribute::parse(parser)?;
				Ok(Component::Attribute(parser.boxup(attr)).spanned(span.until(parser.cur().span)))
			}
			_ => Err(diagnostics::Unimplemented(parser.cur().span))?,
		}
	}
}

impl<'a> WriteCss<'a> for Component<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Type(ty) => {
				sink.write_str(ty)?;
			}
			Self::Id(id) => {
				sink.write_char('#')?;
				sink.write_str(id)?;
			}
			Self::Class(class) => {
				sink.write_char('.')?;
				sink.write_str(class)?;
			}
			Self::PseudoClass(pseudo) => {
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())?;
			}
			Self::LegacyPseudoElement(pseudo) => {
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())?;
			}
			Self::PseudoElement(pseudo) => {
				sink.write_char(':')?;
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())?;
			}
			Self::Attribute(attr) => {
				attr.write_css(sink)?;
			}
			Self::Combinator(combinator) => {
				sink.write_trivia_char(' ')?;
				match combinator {
					Combinator::Descendant => {
						sink.write_char(' ')?;
					}
					Combinator::Child => {
						sink.write_char('>')?;
					}
					Combinator::NextSibling => {
						sink.write_char('+')?;
					}
					Combinator::SubsequentSibling => {
						sink.write_char('~')?;
					}
					Combinator::ColumnCombintor => {
						sink.write_char('|')?;
						sink.write_char('|')?;
					}
				}
				sink.write_trivia_char(' ')?;
			}
			Self::Wildcard => {
				sink.write_char('*')?;
			}
			_ => todo!(),
		}
		Ok(())
	}
}

// https://drafts.csswg.org/css-pseudo/#index-defined-here
#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum PseudoElement {
	After,              // atom!("after")
	Backdrop,           // atom!("backdrop")
	Before,             // atom!("after")
	Cue,                // atom!("cue")
	CueRegion,          // atom!("cue-region")
	FirstLetter,        // atom!("first-letter")
	FirstLine,          // atom!("first-line")
	FileSelectorButton, // atom!("file-selector-button")
	GrammarError,       // atom!("grammar-error")
	Marker,             // atom!("marker")
	Placeholder,        // atom!("placeholder")
	Selection,          // atom!("selection")
	SpellingError,      // atom!("spelling-error")
	TargetText,         // atom!("target-text")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum LegacyPseudoElement {
	After,       // atom!("after")
	Before,      // atom!("before")
	FirstLetter, // atom!("first-letter")
	FirstLine,   // atom!("first-line")
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum PseudoFunction<'a> {
	Dir(DirValue),                // atom!("dir")
	Has(RelativeSelector<'a>),    // atom!("has")
	Host(Selector<'a>),           // atom!("host")
	HostContext(Selector<'a>),    // atom!("host-context")
	Is(ForgivingSelector<'a>),    // atom!("is")
	Lang(Box<'a, Vec<'a, Atom>>), // atom!("lang")
	Not(Selector<'a>),            // atom!("not")
	NthChild(ANBEvenOdd),         // atom!("nth-child")
	NthCol(ANB),                  // atom!("nth-col")
	NthLastChild(ANBEvenOdd),     // atom!("nth-last-child")
	NthLastCol(ANB),              // atom!("nth-last-col")
	NthLastOfType(ANBEvenOdd),    // atom!("nth-last-of-type")
	NthOfType(ANBEvenOdd),        // atom!("nth-of-type")
	Where(ForgivingSelector<'a>), // atom!("where")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum DirValue {
	Rtl, // atom!("rtl")
	Ltr, // atom!("ltr")
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum NSPrefix {
	None,
	Wildcard,
	Named(Atom),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// https://drafts.csswg.org/selectors/#combinators
pub enum Combinator {
	Descendant,        // (Space)
	Child,             // >
	NextSibling,       // +
	SubsequentSibling, // ~
	ColumnCombintor,   // ||
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ANB {
	string: Atom,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ANBEvenOdd {
	string: Atom,
}

pub(crate) fn parse_wq_name(parser: &mut Parser) -> ParserResult<(NSPrefix, Atom)> {
	let peeked = parser.peek();
	let mut nsprefix = NSPrefix::None;
	if peeked.kind == Kind::Delim && peeked.value.as_char().unwrap() == '|' {
		match parser.cur().kind {
			Kind::Delim => {
				let span = parser.cur().span;
				let ch = parser.expect_delim()?;
				if ch == '*' {
					nsprefix = NSPrefix::Wildcard;
				} else {
					Err(diagnostics::UnexpectedDelim(ch, span))?
				}
			}
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				nsprefix = NSPrefix::Named(ident);
			}
			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
		}
		let span = parser.cur().span;
		let ch = parser.expect_delim()?;
		if ch != '|' {
			Err(diagnostics::UnexpectedDelim(ch, span))?
		}
		Ok((nsprefix, parser.expect_ident()?))
	} else {
		if parser.at(Kind::Delim) && parser.cur_char().unwrap() == '|' {
			parser.next_token();
		}
		Ok((nsprefix, parser.expect_ident()?))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Selector>(), 8);
		assert_eq!(::std::mem::size_of::<ForgivingSelector>(), 8);
		assert_eq!(::std::mem::size_of::<RelativeSelector>(), 8);
		assert_eq!(::std::mem::size_of::<Component>(), 24);
		assert_eq!(::std::mem::size_of::<PseudoElement>(), 1);
		assert_eq!(::std::mem::size_of::<LegacyPseudoElement>(), 1);
		assert_eq!(::std::mem::size_of::<PseudoClass>(), 1);
		assert_eq!(::std::mem::size_of::<PseudoFunction>(), 16);
		assert_eq!(::std::mem::size_of::<DirValue>(), 1);
		assert_eq!(::std::mem::size_of::<Combinator>(), 1);
		assert_eq!(::std::mem::size_of::<ANB>(), 8);
		assert_eq!(::std::mem::size_of::<ANBEvenOdd>(), 8);
	}
}
