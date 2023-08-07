use hdx_atom::atom;
use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::{AbsoluteUnit, CSSFloat};
use crate::Writable;

// https://drafts.csswg.org/css-values/#resolution
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Time {
	#[writable(suffix = "ms")]
	Ms(CSSFloat),
	#[writable(suffix = "s")]
	S(CSSFloat),
}

impl Into<CSSFloat> for Time {
	fn into(self) -> CSSFloat {
		match self {
			Self::Ms(f) | Self::S(f) => f,
		}
	}
}

impl AbsoluteUnit for Time {
	fn to_base(&self) -> Self {
		Self::S(match self {
			Self::Ms(f) => *f / 1000.0,
			Self::S(f) => *f,
		})
	}
}

impl FromToken for Time {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			let f: CSSFloat = tok.value.as_f32().unwrap().into();
			let unit = tok.value.as_atom_lower().unwrap();
			match unit {
				atom!("ms") => Ok(Self::Ms(f)),
				atom!("s") => Ok(Self::S(f)),
				_ => Err(diagnostics::UnexpectedDimension(unit, tok.span))?,
			}
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
