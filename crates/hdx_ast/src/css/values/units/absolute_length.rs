use hdx_atom::atom;
use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::{AbsoluteUnit, CSSFloat};
use crate::Writable;

const PX_CM: f32 = PX_IN / 2.54;
const PX_MM: f32 = PX_IN / 25.4;
const PX_Q: f32 = PX_MM / 4.0;
const PX_IN: f32 = 96.0;
const PX_PC: f32 = PX_IN / 6.0;
const PX_PT: f32 = PX_IN / 72.0;

// https://drafts.csswg.org/css-values/#absolute-lengths
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum AbsoluteLength {
	#[writable(suffix = "cm")]
	Cm(CSSFloat),
	#[writable(suffix = "mm")]
	Mm(CSSFloat),
	#[writable(suffix = "q")]
	Q(CSSFloat),
	#[writable(suffix = "in")]
	In(CSSFloat),
	#[writable(suffix = "pc")]
	Pc(CSSFloat),
	#[writable(suffix = "pt")]
	Pt(CSSFloat),
	#[writable(suffix = "px")]
	Px(CSSFloat),
}

impl Into<CSSFloat> for AbsoluteLength {
	fn into(self) -> CSSFloat {
		match self {
			Self::Cm(v)
			| Self::Mm(v)
			| Self::Q(v)
			| Self::In(v)
			| Self::Pc(v)
			| Self::Pt(v)
			| Self::Px(v) => v,
		}
	}
}

impl AbsoluteUnit for AbsoluteLength {
	fn to_base(&self) -> Self {
		Self::Px(match self {
			Self::Cm(f) => *f * PX_CM,
			Self::Mm(f) => *f * PX_MM,
			Self::Q(f) => *f * PX_Q,
			Self::In(f) => *f * PX_IN,
			Self::Pc(f) => *f * PX_PC,
			Self::Pt(f) => *f * PX_PT,
			Self::Px(f) => *f,
		})
	}
}

impl FromToken for AbsoluteLength {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			let f = tok.value.as_f32().unwrap();
			let unit = tok.value.as_atom_lower().unwrap();
			match unit {
				atom!("cm") => Ok(Self::Cm(f.into())),
				atom!("mm") => Ok(Self::Mm(f.into())),
				atom!("q") => Ok(Self::Q(f.into())),
				atom!("in") => Ok(Self::In(f.into())),
				atom!("pc") => Ok(Self::Pc(f.into())),
				atom!("pt") => Ok(Self::Pt(f.into())),
				atom!("px") => Ok(Self::Px(f.into())),
				_ => Err(diagnostics::UnexpectedDimension(unit, tok.span))?,
			}
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
