use hdx_atom::Atomizable;
use hdx_derive::{Atomizable, Value, Writable};
use hdx_lexer::Token;
use hdx_parser::{discard, unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[derive(Value, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AnimationFillMode(pub SmallVec<[SingleAnimationFillMode; 8]>);

#[derive(Atomizable, Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SingleAnimationFillMode {
	#[default]
	None, // atom!("none")
	Forwards,  // atom!("forwards")
	Backwards, // atom!("backwards")
	Both,      // atom!("both")
}

impl<'a> Parse<'a> for AnimationFillMode {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = smallvec![];
		loop {
			match parser.next() {
				Token::Ident(atom) => {
					if let Some(fill) = SingleAnimationFillMode::from_atom(&atom) {
						values.push(fill);
					} else {
						unexpected_ident!(parser, atom);
					}
				}
				token => unexpected!(parser, token),
			}
			if !discard!(parser, Token::Comma) {
				break;
			}
		}
		Ok(Self(values))
	}
}

impl<'a> WriteCss<'a> for AnimationFillMode {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.0.iter().peekable();
		while let Some(fill) = iter.next() {
			fill.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			}
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
		assert_size!(AnimationFillMode, 24);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnimationFillMode, "both");
		assert_parse!(AnimationFillMode, "none, both, backwards, forwards");
	}

	#[test]
	fn test_minify() {
		assert_minify!(AnimationFillMode, "none, both, backwards, forwards", "none,both,backwards,forwards");
	}
}
