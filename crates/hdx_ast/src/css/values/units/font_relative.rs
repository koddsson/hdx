use hdx_atom::atom;
use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::CSSFloat;
use crate::Writable;

// https://drafts.csswg.org/css-values/#font-relative-lengths
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FontRelative {
	#[writable(suffix = "em")]
	Em(CSSFloat),
	#[writable(suffix = "rem")]
	Rem(CSSFloat),
	#[writable(suffix = "ex")]
	Ex(CSSFloat),
	#[writable(suffix = "rex")]
	Rex(CSSFloat),
	#[writable(suffix = "cap")]
	Cap(CSSFloat),
	#[writable(suffix = "rcap")]
	Rcap(CSSFloat),
	#[writable(suffix = "ch")]
	Ch(CSSFloat),
	#[writable(suffix = "rch")]
	Rch(CSSFloat),
	#[writable(suffix = "ic")]
	Ic(CSSFloat),
	#[writable(suffix = "ric")]
	Ric(CSSFloat),
	#[writable(suffix = "lh")]
	Lh(CSSFloat),
	#[writable(suffix = "rlh")]
	Rlh(CSSFloat),
}

impl Into<CSSFloat> for FontRelative {
	fn into(self) -> CSSFloat {
		match self {
			Self::Em(f)
			| Self::Rem(f)
			| Self::Ex(f)
			| Self::Rex(f)
			| Self::Cap(f)
			| Self::Rcap(f)
			| Self::Ch(f)
			| Self::Rch(f)
			| Self::Ic(f)
			| Self::Ric(f)
			| Self::Lh(f)
			| Self::Rlh(f) => f,
		}
	}
}

impl FromToken for FontRelative {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			let f: CSSFloat = tok.value.as_f32().unwrap().into();
			let unit = tok.value.as_atom_lower().unwrap();
			match unit {
				atom!("em") => Ok(Self::Em(f)),
				atom!("rem") => Ok(Self::Rem(f)),
				atom!("ex") => Ok(Self::Ex(f)),
				atom!("rex") => Ok(Self::Rex(f)),
				atom!("cap") => Ok(Self::Cap(f)),
				atom!("rcap") => Ok(Self::Rcap(f)),
				atom!("ch") => Ok(Self::Ch(f)),
				atom!("rch") => Ok(Self::Rch(f)),
				atom!("ic") => Ok(Self::Ic(f)),
				atom!("ric") => Ok(Self::Ric(f)),
				atom!("lh") => Ok(Self::Lh(f)),
				atom!("rlh") => Ok(Self::Rlh(f)),
				_ => Err(diagnostics::UnexpectedDimension(unit, tok.span))?,
			}
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
