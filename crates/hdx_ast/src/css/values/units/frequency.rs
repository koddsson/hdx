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
pub enum Frequency {
	#[writable(suffix = "hz")]
	Hz(CSSFloat),
	#[writable(suffix = "khz")]
	Khz(CSSFloat),
}

impl Into<CSSFloat> for Frequency {
	fn into(self) -> CSSFloat {
		match self {
			Self::Hz(f) | Self::Khz(f) => f,
		}
	}
}

impl AbsoluteUnit for Frequency {
	fn to_base(&self) -> Self {
		Self::Hz(match self {
			Self::Khz(f) => *f * 1000.0,
			Self::Hz(f) => *f,
		})
	}
}

impl FromToken for Frequency {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			let f: CSSFloat = tok.value.as_f32().unwrap().into();
			let unit = tok.value.as_atom_lower().unwrap();
			match unit {
				atom!("khz") => Ok(Self::Khz(f)),
				atom!("hz") => Ok(Self::Hz(f)),
				_ => Err(diagnostics::UnexpectedDimension(unit, tok.span))?,
			}
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
