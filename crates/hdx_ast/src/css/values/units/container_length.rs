use hdx_atom::atom;
use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::CSSFloat;

// https://www.w3.org/TR/css-contain-3/#container-lengths
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ContainerLength {
	Cqw(CSSFloat),
	Cqh(CSSFloat),
	Cqi(CSSFloat),
	Cqb(CSSFloat),
	Cqmin(CSSFloat),
	Cqmax(CSSFloat),
}

impl Into<CSSFloat> for ContainerLength {
	fn into(self) -> CSSFloat {
		match self {
			Self::Cqw(f)
			| Self::Cqh(f)
			| Self::Cqi(f)
			| Self::Cqb(f)
			| Self::Cqmin(f)
			| Self::Cqmax(f) => f,
		}
	}
}

impl FromToken for ContainerLength {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			let f: CSSFloat = tok.value.as_f32().unwrap().into();
			let unit = tok.value.as_atom_lower().unwrap();
			match unit {
				atom!("cqw") => Ok(Self::Cqw(f)),
				atom!("cqh") => Ok(Self::Cqh(f)),
				atom!("cqi") => Ok(Self::Cqi(f)),
				atom!("cqb") => Ok(Self::Cqb(f)),
				atom!("cqmin") => Ok(Self::Cqmin(f)),
				atom!("cqmax") => Ok(Self::Cqmax(f)),
				_ => Err(diagnostics::UnexpectedDimension(unit, tok.span))?,
			}
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
