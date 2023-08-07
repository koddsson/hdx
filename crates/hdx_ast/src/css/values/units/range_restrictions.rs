use hdx_lexer::Token;
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::Unit;

// See https://drafts.csswg.org/css-values-4/#numeric-ranges which describes possible range
// restrictions on units.

// Positive is a generic struct representing any "Unit", but making the additional parser check to
// ensure that the f32 attached to that unit is not negative. This represents the CSS spec notation
// of <T [0,∞]>
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Positive<T: Unit + FromToken>(T);

impl<T: Unit + FromToken> FromToken for Positive<T> {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		let v = T::from_token(tok)?;
		if v.is_negative() {
			Err(diagnostics::NumberOutOfBounds(0.0, f32::MAX, tok.span))?
		} else {
			Ok(Self(v))
		}
	}
}

impl<'a, T: WriteCss<'a> + Unit + FromToken> WriteCss<'a> for Positive<T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)
	}
}
