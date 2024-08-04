use hdx_lexer::Include;
use hdx_parser::{diagnostics, discard, Delim, Parse, Parser, Result as ParserResult, Token};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
// https://drafts.csswg.org/selectors/#combinators
pub enum Combinator {
	Descendant,        // (Space)
	Child,             // >
	NextSibling,       // +
	SubsequentSibling, // ~
	Column,            // ||
	Nesting,           // &
}

impl<'a> Parse<'a> for Combinator {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let could_be_descendant_combinator = discard!(parser, Include::Whitespace, Whitespace);
		if let Some(token) = parser.peek::<Token![Delim]>() {
			let char = token.char();
			if could_be_descendant_combinator && !matches!(char, Some('>' | '+' | '~' | '|')) {
				return Ok(Self::Descendant);
			}
			let val = match char {
				Some('>') => Self::Child,
				Some('+') => Self::NextSibling,
				Some('~') => Self::SubsequentSibling,
				Some('&') => Self::Nesting,
				Some('|') => {
					parser.hop(token);
					parser.parse_with::<Delim![|]>(Include::Whitespace)?;
					return Ok(Self::Column);
				}
				_ if could_be_descendant_combinator => return Ok(Self::Descendant),
				_ => Err(diagnostics::Unexpected(token, token.span()))?,
			};
			parser.hop(token);
			if val != Self::Nesting {
				discard!(parser, Include::Whitespace, Whitespace);
			}
			Ok(val)
		} else if could_be_descendant_combinator {
			loop {
				if !discard!(parser, Include::Whitespace, Whitespace) {
					break;
				}
			}
			Ok(Self::Descendant)
		} else {
			let token = parser.peek::<Token![Any]>().unwrap();
			Err(diagnostics::Unexpected(token, token.span()))?
		}
	}
}

impl<'a> WriteCss<'a> for Combinator {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Descendant => sink.write_char(' ')?,
			Self::Nesting => write_css!(sink, '&'),
			Self::Child => write_css!(sink, (), '>', ()),
			Self::NextSibling => write_css!(sink, (), '+', ()),
			Self::SubsequentSibling => write_css!(sink, (), '~', ()),
			Self::Column => write_css!(sink, (), '|', '|', ()),
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Combinator, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Combinator, ">", " > ");
		assert_parse!(Combinator, "+", " + ");
		assert_parse!(Combinator, "~", " ~ ");
		assert_parse!(Combinator, "&", "&");
		// Descendent combinator
		assert_parse!(Combinator, "     ", " ");
		assert_parse!(Combinator, "     ", " ");
		assert_parse!(Combinator, "  /**/   /**/   /**/ ", " ");
		// Column
		assert_parse!(Combinator, "||", " || ");
		assert_parse!(Combinator, " || ", " || ");
	}
}
