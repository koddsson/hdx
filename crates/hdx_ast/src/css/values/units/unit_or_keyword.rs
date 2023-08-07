use hdx_atom::Atomizable;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::Unit;

// Often the spec will detail additional keywords allowable aside from a unit, for example
// margins can be a `<length-percentage> | auto`.
// (https://drafts.csswg.org/css-box-4/#propdef-margin-left)
// This generic struct allows a combination of an atomizable Enum _or_ a "Unit" in combination, to
// make it easy to express these unit types.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum UnitOrKeyword<K: Atomizable, U: Unit + FromToken> {
	Unit(U),
	Keyword(K),
}

impl<T: Unit + FromToken> FromToken for UnitOrKeyword<T> {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		let v = T::from_token(tok)?;
		if v.is_negative() {
			Err(diagnostics::NumberOutOfBounds(0.0, f32::MAX, tok.span))?
		} else {
			Ok(Self(v))
		}
	}
}

impl<'a, T: WriteCss<'a> + Unit + FromToken> WriteCss<'a> for UnitOrKeyword<T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Unit(u) => u.write_css(sink),
			Self::Keyword(k) => k.write_css(sink),
		}
	}
}
