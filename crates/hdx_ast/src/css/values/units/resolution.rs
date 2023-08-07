use hdx_atom::atom;
use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

const DPPX_IN: f32 = 96.0;
const DPPX_CM: f32 = DPPX_IN / 2.54;

use super::{AbsoluteUnit, CSSFloat};
use crate::Writable;

// https://drafts.csswg.org/css-values/#resolution
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Resolution {
	#[writable(suffix = "dpi")]
	Dpi(CSSFloat),
	#[writable(suffix = "dpcm")]
	Dpcm(CSSFloat),
	#[writable(suffix = "dppx")]
	Dppx(CSSFloat),
}

impl Into<CSSFloat> for Resolution {
	fn into(self) -> CSSFloat {
		match self {
			Self::Dpi(f) | Self::Dpcm(f) | Self::Dppx(f) => f,
		}
	}
}

impl AbsoluteUnit for Resolution {
	fn to_base(&self) -> Self {
		Self::Dppx(match self {
			Self::Dpi(f) => *f * DPPX_IN,
			Self::Dpcm(f) => *f * DPPX_CM,
			Self::Dppx(f) => *f,
		})
	}
}

impl FromToken for Resolution {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			let f: CSSFloat = tok.value.as_f32().unwrap().into();
			let unit = tok.value.as_atom_lower().unwrap();
			match unit {
				atom!("dpi") => Ok(Self::Dpi(f)),
				atom!("dpcm") => Ok(Self::Dpcm(f)),
				atom!("dppx") => Ok(Self::Dppx(f)),
				_ => Err(diagnostics::UnexpectedDimension(unit, tok.span))?,
			}
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
