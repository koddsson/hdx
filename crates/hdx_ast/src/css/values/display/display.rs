use hdx_atom::atom;
use hdx_lexer::{Kind, Spanned};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{bitmask, Atomizable};

// https://drafts.csswg.org/css-display-4/#propdef-display
#[derive(Default, Atomizable)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum DisplayInside {
	#[default]
	Flow = 0b0, // atom!("flow")
	FlowRoot = 0b1, // atom!("flow-root")
	Table = 0b10,   // atom!("table")
	Flex = 0b100,   // atom!("flex")
	Grid = 0b1000,  // atom!("grid")
	Ruby = 0b10000, // atom!("ruby")
}

impl DisplayInside {
	fn new(u: u8) -> Self {
		Self { bits: 0b11111 & u }
	}
}

// https://drafts.csswg.org/css-display-4/#propdef-display
#[derive(Default, Atomizable)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum DisplayOutside {
	#[default]
	// Display inside takes the first 5 so this must use bits 7 & 8
	Inline = 0b100000,
	Block = 0b1000000,
	RunIn = 0b10000000,
}

impl DisplayOutside {
	fn new(u: u8) -> Self {
		Self { bits: 0b11100000 & u }
	}
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Display {
	// <display-box>
	None,
	Contents,
	// <display-internal>
	TableRowGroup,
	TableHeaderGroup,
	TableFooterGroup,
	TableRow,
	TableCell,
	TableColumnGroup,
	TableColumn,
	TableCaption,
	RubyBase,
	RubyText,
	RubyBaseContainer,
	RubyTextContainer,
	// <display-listitem>
	ListItem,
	ListItemOutside(DisplayOutside),
	ListItemFlow,
	ListItemFlowOutside(DisplayOutside),
	ListItemFlowRoot,
	ListItemFlowRootOutside(DisplayOutside),

	// Omitted Inside
	Block,
	#[default]
	Inline,
	RunIn,

	// Omitted Outside
	Flow,
	FlowRoot,
	Table,
	Flex,
	Grid,
	Ruby,

	// <display-legacy>
	InlineBlock,
	InlineTable,
	InlineFlex,
	InlineGrid,

	// Inside & Outside
	Pair(u8),
}
//
impl Display {
	// https://drafts.csswg.org/css-display-4/#display-value-summary
	pub fn expand(&self) -> Self {
		match self {
			Self::Block => Self::pair_from(DisplayOutside::Block, DisplayInside::Flow),
			Self::Inline => Self::pair_from(DisplayOutside::Inline, DisplayInside::Flow),
			Self::RunIn => Self::pair_from(DisplayOutside::RunIn, DisplayInside::Flow),
			Self::ListItem => Self::ListItemFlowOutside(DisplayOutside::Block),
			Self::Flow => Self::pair_from(DisplayOutside::Block, DisplayInside::Flow),
			Self::FlowRoot => Self::pair_from(DisplayOutside::Block, DisplayInside::FlowRoot),
			Self::Table => Self::pair_from(DisplayOutside::Block, DisplayInside::Table),
			Self::Flex => Self::pair_from(DisplayOutside::Block, DisplayInside::Flex),
			Self::Grid => Self::pair_from(DisplayOutside::Block, DisplayInside::Grid),
			Self::Ruby => Self::pair_from(DisplayOutside::Inline, DisplayInside::Grid),
			Self::InlineBlock => Self::pair_from(DisplayOutside::Inline, DisplayInside::FlowRoot),
			Self::InlineTable => Self::pair_from(DisplayOutside::Inline, DisplayInside::Table),
			Self::InlineFlex => Self::pair_from(DisplayOutside::Inline, DisplayInside::Flex),
			Self::InlineGrid => Self::pair_from(DisplayOutside::Inline, DisplayInside::Grid),
			Self::ListItemOutside(u) => Self::ListItemFlowOutside(*u),
			_ => *self,
		}
	}

	pub fn contract(&self) -> Self {
		match self {
			Self::Pair(u) => match *u {
				u if u == DisplayOutside::Block.bits | DisplayInside::Flow.bits => Self::Block,
				u if u == DisplayOutside::Inline.bits | DisplayInside::Flow.bits => Self::Inline,
				u if u == DisplayOutside::RunIn.bits | DisplayInside::Flow.bits => Self::RunIn,
				u if u == DisplayOutside::Block.bits | DisplayInside::Flow.bits => Self::Flow,
				u if u == DisplayOutside::Block.bits | DisplayInside::FlowRoot.bits => {
					Self::FlowRoot
				}
				u if u == DisplayOutside::Block.bits | DisplayInside::Table.bits => Self::Table,
				u if u == DisplayOutside::Block.bits | DisplayInside::Flex.bits => Self::Flex,
				u if u == DisplayOutside::Block.bits | DisplayInside::Grid.bits => Self::Grid,
				u if u == DisplayOutside::Inline.bits | DisplayInside::Ruby.bits => Self::Ruby,
				u if u == DisplayOutside::Inline.bits | DisplayInside::FlowRoot.bits => {
					Self::InlineBlock
				}
				u if u == DisplayOutside::Inline.bits | DisplayInside::Table.bits => {
					Self::InlineTable
				}
				u if u == DisplayOutside::Inline.bits | DisplayInside::Flex.bits => {
					Self::InlineFlex
				}
				u if u == DisplayOutside::Inline.bits | DisplayInside::Grid.bits => {
					Self::InlineGrid
				}
				_ => *self,
			},
			Self::ListItemFlowOutside(u) => match *u {
				DisplayOutside::Block => Self::ListItem,
				o => Self::ListItemOutside(o),
			},
			_ => *self,
		}
	}

	pub fn pair_from(outside: DisplayOutside, inside: DisplayInside) -> Self {
		Self::Pair(outside.bits | inside.bits)
	}
}

impl<'a> Parse<'a> for Display {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.cur().span;
		let value = match parser.expect_ident()? {
			// <display-box>
			atom!("none") => Display::None,
			atom!("contents") => Display::Contents,
			// <display-internal>
			atom!("table-row-group") => Display::TableRowGroup,
			atom!("table-header-group") => Display::TableHeaderGroup,
			atom!("table-footer-group") => Display::TableFooterGroup,
			atom!("table-row") => Display::TableRow,
			atom!("table-cell") => Display::TableCell,
			atom!("table-column-group") => Display::TableColumnGroup,
			atom!("table-column") => Display::TableColumn,
			atom!("table-caption") => Display::TableCaption,
			atom!("ruby-base") => Display::RubyBase,
			atom!("ruby-text") => Display::RubyText,
			atom!("ruby-base-container") => Display::RubyBaseContainer,
			atom!("ruby-text-container") => Display::RubyTextContainer,
			// <display-legacy>
			atom!("inline-block") => Display::InlineBlock,
			atom!("inline-table") => Display::InlineTable,
			atom!("inline-flex") => Display::InlineFlex,
			atom!("inline-grid") => Display::InlineGrid,
			id1 => {
				let id2 = if parser.at(Kind::Ident) { parser.expect_ident()? } else { atom!("") };
				let id3 = if parser.at(Kind::Ident) { parser.expect_ident()? } else { atom!("") };
				match (id1.clone(), id2, id3) {
					// Singular Idents
					(atom!("block"), atom!(""), atom!("")) => Display::Block,
					(atom!("inline"), atom!(""), atom!("")) => Display::Inline,
					(atom!("run-in"), atom!(""), atom!("")) => Display::RunIn,
					(atom!("flow"), atom!(""), atom!("")) => Display::Flow,
					(atom!("flow-root"), atom!(""), atom!("")) => Display::FlowRoot,
					(atom!("table"), atom!(""), atom!("")) => Display::Table,
					(atom!("flex"), atom!(""), atom!("")) => Display::Flex,
					(atom!("grid"), atom!(""), atom!("")) => Display::Grid,
					(atom!("ruby"), atom!(""), atom!("")) => Display::Ruby,
					(atom!("list-item"), atom!(""), atom!("")) => Display::ListItem,

					// Block Pairs
					(atom!("block"), atom!("flow"), atom!(""))
					| (atom!("flow"), atom!("block"), atom!("")) => {
						Display::pair_from(DisplayOutside::Block, DisplayInside::Flow)
					}
					(atom!("block"), atom!("flow-root"), atom!(""))
					| (atom!("flow-root"), atom!("block"), atom!("")) => {
						Display::pair_from(DisplayOutside::Block, DisplayInside::FlowRoot)
					}
					(atom!("block"), atom!("table"), atom!(""))
					| (atom!("table"), atom!("block"), atom!("")) => {
						Display::pair_from(DisplayOutside::Block, DisplayInside::Table)
					}
					(atom!("block"), atom!("flex"), atom!(""))
					| (atom!("flex"), atom!("block"), atom!("")) => {
						Display::pair_from(DisplayOutside::Block, DisplayInside::Flex)
					}
					(atom!("block"), atom!("grid"), atom!(""))
					| (atom!("grid"), atom!("block"), atom!("")) => {
						Display::pair_from(DisplayOutside::Block, DisplayInside::Grid)
					}
					(atom!("block"), atom!("ruby"), atom!(""))
					| (atom!("ruby"), atom!("block"), atom!("")) => {
						Display::pair_from(DisplayOutside::Block, DisplayInside::Ruby)
					}
					(atom!("list-item"), atom!("block"), atom!(""))
					| (atom!("block"), atom!("list-item"), atom!("")) => {
						Display::ListItemOutside(DisplayOutside::Block)
					}

					// Inline Pairs
					(atom!("inline"), atom!("flow"), atom!(""))
					| (atom!("flow"), atom!("inline"), atom!("")) => {
						Display::pair_from(DisplayOutside::Inline, DisplayInside::Flow)
					}
					(atom!("inline"), atom!("flow-root"), atom!(""))
					| (atom!("flow-root"), atom!("inline"), atom!("")) => {
						Display::pair_from(DisplayOutside::Inline, DisplayInside::FlowRoot)
					}
					(atom!("inline"), atom!("table"), atom!(""))
					| (atom!("table"), atom!("inline"), atom!("")) => {
						Display::pair_from(DisplayOutside::Inline, DisplayInside::Table)
					}
					(atom!("inline"), atom!("flex"), atom!(""))
					| (atom!("flex"), atom!("inline"), atom!("")) => {
						Display::pair_from(DisplayOutside::Inline, DisplayInside::Flex)
					}
					(atom!("inline"), atom!("grid"), atom!(""))
					| (atom!("grid"), atom!("inline"), atom!("")) => {
						Display::pair_from(DisplayOutside::Inline, DisplayInside::Grid)
					}
					(atom!("inline"), atom!("ruby"), atom!(""))
					| (atom!("ruby"), atom!("inline"), atom!("")) => {
						Display::pair_from(DisplayOutside::Inline, DisplayInside::Ruby)
					}
					(atom!("list-item"), atom!("inline"), atom!(""))
					| (atom!("inline"), atom!("list-item"), atom!("")) => {
						Display::ListItemOutside(DisplayOutside::Inline)
					}

					// Run-In Pairs
					(atom!("run-in"), atom!("flow"), atom!(""))
					| (atom!("flow"), atom!("run-in"), atom!("")) => {
						Display::pair_from(DisplayOutside::RunIn, DisplayInside::Flow)
					}
					(atom!("run-in"), atom!("flow-root"), atom!(""))
					| (atom!("flow-root"), atom!("run-in"), atom!("")) => {
						Display::pair_from(DisplayOutside::RunIn, DisplayInside::FlowRoot)
					}
					(atom!("run-in"), atom!("table"), atom!(""))
					| (atom!("table"), atom!("run-in"), atom!("")) => {
						Display::pair_from(DisplayOutside::RunIn, DisplayInside::Table)
					}
					(atom!("run-in"), atom!("flex"), atom!(""))
					| (atom!("flex"), atom!("run-in"), atom!("")) => {
						Display::pair_from(DisplayOutside::RunIn, DisplayInside::Flex)
					}
					(atom!("run-in"), atom!("grid"), atom!(""))
					| (atom!("grid"), atom!("run-in"), atom!("")) => {
						Display::pair_from(DisplayOutside::RunIn, DisplayInside::Grid)
					}
					(atom!("run-in"), atom!("ruby"), atom!(""))
					| (atom!("ruby"), atom!("run-in"), atom!("")) => {
						Display::pair_from(DisplayOutside::RunIn, DisplayInside::Ruby)
					}
					(atom!("list-item"), atom!("run-in"), atom!(""))
					| (atom!("run-in"), atom!("list-item"), atom!("")) => {
						Display::ListItemOutside(DisplayOutside::RunIn)
					}

					// ListItem Triple (Flow)
					(atom!("list-item"), atom!("flow"), atom!("block"))
					| (atom!("flow"), atom!("list-item"), atom!("block"))
					| (atom!("list-item"), atom!("block"), atom!("flow"))
					| (atom!("flow"), atom!("block"), atom!("list-item"))
					| (atom!("block"), atom!("list-item"), atom!("flow"))
					| (atom!("block"), atom!("flow"), atom!("list-item")) => {
						Display::ListItemFlowOutside(DisplayOutside::Block)
					}
					(atom!("list-item"), atom!("flow"), atom!("inline"))
					| (atom!("flow"), atom!("list-item"), atom!("inline"))
					| (atom!("list-item"), atom!("inline"), atom!("flow"))
					| (atom!("flow"), atom!("inline"), atom!("list-item"))
					| (atom!("inline"), atom!("list-item"), atom!("flow"))
					| (atom!("inline"), atom!("flow"), atom!("list-item")) => {
						Display::ListItemFlowOutside(DisplayOutside::Inline)
					}
					(atom!("list-item"), atom!("flow"), atom!("run-in"))
					| (atom!("flow"), atom!("list-item"), atom!("run-in"))
					| (atom!("list-item"), atom!("run-in"), atom!("flow"))
					| (atom!("flow"), atom!("run-in"), atom!("list-item"))
					| (atom!("run-in"), atom!("list-item"), atom!("flow"))
					| (atom!("run-in"), atom!("flow"), atom!("list-item")) => {
						Display::ListItemFlowOutside(DisplayOutside::RunIn)
					}

					// ListItem Triple (FlowRoot)
					(atom!("list-item"), atom!("flow-root"), atom!("block"))
					| (atom!("flow-root"), atom!("list-item"), atom!("block"))
					| (atom!("list-item"), atom!("block"), atom!("flow-root"))
					| (atom!("flow-root"), atom!("block"), atom!("list-item"))
					| (atom!("block"), atom!("list-item"), atom!("flow-root"))
					| (atom!("block"), atom!("flow-root"), atom!("list-item")) => {
						Display::ListItemFlowRootOutside(DisplayOutside::Block)
					}
					(atom!("list-item"), atom!("flow-root"), atom!("inline"))
					| (atom!("flow-root"), atom!("list-item"), atom!("inline"))
					| (atom!("list-item"), atom!("inline"), atom!("flow-root"))
					| (atom!("flow-root"), atom!("inline"), atom!("list-item"))
					| (atom!("inline"), atom!("list-item"), atom!("flow-root"))
					| (atom!("inline"), atom!("flow-root"), atom!("list-item")) => {
						Display::ListItemFlowRootOutside(DisplayOutside::Inline)
					}
					(atom!("list-item"), atom!("flow-root"), atom!("run-in"))
					| (atom!("flow-root"), atom!("list-item"), atom!("run-in"))
					| (atom!("list-item"), atom!("run-in"), atom!("flow-root"))
					| (atom!("flow-root"), atom!("run-in"), atom!("list-item"))
					| (atom!("run-in"), atom!("list-item"), atom!("flow-root"))
					| (atom!("run-in"), atom!("flow-root"), atom!("list-item")) => {
						Display::ListItemFlowRootOutside(DisplayOutside::RunIn)
					}
					_ => Err(diagnostics::UnexpectedIdent(id1, span))?,
				}
			}
		};
		Ok(value.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> WriteCss<'a> for Display {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self.contract() {
			Self::None => sink.write_str("none"),
			Self::Contents => sink.write_str("contents"),
			Self::TableRowGroup => sink.write_str("table-row-group"),
			Self::TableHeaderGroup => sink.write_str("table-header-group"),
			Self::TableFooterGroup => sink.write_str("table-footer-group"),
			Self::TableRow => sink.write_str("table-row"),
			Self::TableCell => sink.write_str("table-cell"),
			Self::TableColumnGroup => sink.write_str("table-column-group"),
			Self::TableColumn => sink.write_str("table-column"),
			Self::TableCaption => sink.write_str("table-caption"),
			Self::RubyBase => sink.write_str("ruby-base"),
			Self::RubyText => sink.write_str("ruby-text"),
			Self::RubyBaseContainer => sink.write_str("ruby-base-container"),
			Self::RubyTextContainer => sink.write_str("ruby-text-container"),
			Self::ListItem => sink.write_str("list-item"),
			Self::ListItemFlow => sink.write_str("flow list-item"),
			Self::ListItemFlowRoot => sink.write_str("flow-root list-item"),
			Self::ListItemOutside(u) => {
				u.to_atom().write_css(sink)?;
				sink.write_char(' ')?;
				sink.write_str("list-item")
			}
			Self::ListItemFlowOutside(u) => {
				u.to_atom().write_css(sink)?;
				sink.write_str(" flow ")?;
				sink.write_str("list-item")
			}
			Self::ListItemFlowRootOutside(u) => {
				u.to_atom().write_css(sink)?;
				sink.write_str(" flow-root ")?;
				sink.write_str("list-item")
			}
			Self::Block => sink.write_str("block"),
			Self::Inline => sink.write_str("inline"),
			Self::RunIn => sink.write_str("run-in"),
			Self::Flow => sink.write_str("flow"),
			Self::FlowRoot => sink.write_str("flow-root"),
			Self::Table => sink.write_str("table"),
			Self::Flex => sink.write_str("flex"),
			Self::Grid => sink.write_str("grid"),
			Self::Ruby => sink.write_str("ruby"),
			Self::InlineBlock => sink.write_str("inline-block"),
			Self::InlineTable => sink.write_str("inline-table"),
			Self::InlineFlex => sink.write_str("inline-flex"),
			Self::InlineGrid => sink.write_str("inline-grid"),
			Self::Pair(u) => {
				eprintln!("{:0b}", u);
				dbg!(DisplayOutside::all());
				dbg!(DisplayOutside::new(u) == DisplayOutside::Block);
				dbg!(DisplayOutside::new(u) == DisplayOutside::Inline);
				dbg!(DisplayOutside::new(u) == DisplayOutside::RunIn);
				DisplayOutside::new(u).to_atom().write_css(sink)?;
				sink.write_char(' ')?;
				DisplayInside::new(u).to_atom().write_css(sink)
			}
		}
	}
}

#[cfg(test)]
mod tests {

	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Display>(), 2);
		assert_eq!(::std::mem::size_of::<DisplayInside>(), 1);
		assert_eq!(::std::mem::size_of::<DisplayOutside>(), 1);
	}

	#[test]
	fn test_variants() {
		let allocator = Allocator::default();
		test_write::<Display>(&allocator, "none", "none");
		test_write::<Display>(&allocator, "contents", "contents");
		test_write::<Display>(&allocator, "block flow", "block");
		test_write::<Display>(&allocator, "block flow-root", "flow-root");
		test_write::<Display>(&allocator, "inline flow", "inline");
		test_write::<Display>(&allocator, "inline flow-root", "inline-block");
		test_write::<Display>(&allocator, "run-in flow", "run-in");
		test_write::<Display>(&allocator, "block flow list-item", "list-item");
		test_write::<Display>(&allocator, "inline flow list-item", "inline list-item");
		test_write::<Display>(&allocator, "block flex", "flex");
		test_write::<Display>(&allocator, "inline flex", "inline-flex");
		test_write::<Display>(&allocator, "block grid", "grid");
		test_write::<Display>(&allocator, "inline grid", "inline-grid");
		test_write::<Display>(&allocator, "inline ruby", "ruby");
		test_write::<Display>(&allocator, "block ruby", "block ruby");
		test_write::<Display>(&allocator, "block table", "table");
		test_write::<Display>(&allocator, "inline table", "inline-table");
	}
}
