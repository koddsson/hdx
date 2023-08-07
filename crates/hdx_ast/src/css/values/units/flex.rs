use hdx_atom::atom;
use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::CSSFloat;
use crate::Writable;

// https://www.w3.org/TR/css3-grid-layout/#fr-unit
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
#[writable(suffix = "fr")]
pub struct Flex(CSSFloat);

impl Into<CSSFloat> for Flex {
	fn into(self) -> CSSFloat {
		self.0
	}
}

impl FromToken for Flex {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			let f: CSSFloat = tok.value.as_f32().unwrap().into();
			let unit = tok.value.as_atom_lower().unwrap();
			match unit {
				atom!("fr") => Ok(Self(f)),
				_ => Err(diagnostics::UnexpectedDimension(unit, tok.span))?,
			}
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
