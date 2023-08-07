use hdx_atom::atom;
use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

const DEG_GRAD: f32 = 0.9;
const DEG_RAD: f32 = 57.29577951308232;
const DEG_TURN: f32 = 360.0;

use super::{AbsoluteUnit, CSSFloat};
use crate::Writable;

// https://drafts.csswg.org/css-values/#resolution
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Angle {
	#[writable(suffix = "grad")]
	Grad(CSSFloat),
	#[writable(suffix = "rad")]
	Rad(CSSFloat),
	#[writable(suffix = "turn")]
	Turn(CSSFloat),
	#[writable(suffix = "deg")]
	Deg(CSSFloat),
}

impl Into<CSSFloat> for Angle {
	fn into(self) -> CSSFloat {
		match self {
			Self::Grad(f) | Self::Rad(f) | Self::Turn(f) | Self::Deg(f) => f,
		}
	}
}

impl AbsoluteUnit for Angle {
	fn to_base(&self) -> Self {
		Self::Deg(match self {
			Self::Grad(f) => *f * DEG_GRAD,
			Self::Rad(f) => *f * DEG_RAD,
			Self::Turn(f) => *f * DEG_TURN,
			Self::Deg(f) => *f,
		})
	}
}

impl FromToken for Angle {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			let f: CSSFloat = tok.value.as_f32().unwrap().into();
			let unit = tok.value.as_atom_lower().unwrap();
			match unit {
				atom!("deg") => Ok(Self::Deg(f)),
				atom!("grad") => Ok(Self::Grad(f)),
				atom!("rad") => Ok(Self::Rad(f)),
				atom!("turn") => Ok(Self::Turn(f)),
				_ => Err(diagnostics::UnexpectedDimension(unit, tok.span))?,
			}
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
