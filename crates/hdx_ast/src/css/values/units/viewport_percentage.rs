use hdx_atom::atom;
use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::CSSFloat;
use crate::Writable;

// 	// https://drafts.csswg.org/css-values/#viewport-relative-units
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ViewportPercentage {
	#[writable(suffix = "vw")]
	Vw(CSSFloat),
	#[writable(suffix = "svw")]
	Svw(CSSFloat),
	#[writable(suffix = "lvw")]
	Lvw(CSSFloat),
	#[writable(suffix = "dvw")]
	Dvw(CSSFloat),
	#[writable(suffix = "vh")]
	Vh(CSSFloat),
	#[writable(suffix = "svh")]
	Svh(CSSFloat),
	#[writable(suffix = "lvh")]
	Lvh(CSSFloat),
	#[writable(suffix = "dvh")]
	Dvh(CSSFloat),
	#[writable(suffix = "vi")]
	Vi(CSSFloat),
	#[writable(suffix = "svi")]
	Svi(CSSFloat),
	#[writable(suffix = "lvi")]
	Lvi(CSSFloat),
	#[writable(suffix = "dvi")]
	Dvi(CSSFloat),
	#[writable(suffix = "vb")]
	Vb(CSSFloat),
	#[writable(suffix = "svb")]
	Svb(CSSFloat),
	#[writable(suffix = "lvb")]
	Lvb(CSSFloat),
	#[writable(suffix = "dvb")]
	Dvb(CSSFloat),
	#[writable(suffix = "vmin")]
	Vmin(CSSFloat),
	#[writable(suffix = "svmin")]
	Svmin(CSSFloat),
	#[writable(suffix = "lvmin")]
	Lvmin(CSSFloat),
	#[writable(suffix = "dvmin")]
	Dvmin(CSSFloat),
	#[writable(suffix = "vmax")]
	Vmax(CSSFloat),
	#[writable(suffix = "svmax")]
	Svmax(CSSFloat),
	#[writable(suffix = "lvmax")]
	Lvmax(CSSFloat),
	#[writable(suffix = "dvmax")]
	Dvmax(CSSFloat),
}

impl Into<CSSFloat> for ViewportPercentage {
	fn into(self) -> CSSFloat {
		match self {
			Self::Vw(f)
			| Self::Svw(f)
			| Self::Lvw(f)
			| Self::Dvw(f)
			| Self::Vh(f)
			| Self::Svh(f)
			| Self::Lvh(f)
			| Self::Dvh(f)
			| Self::Vi(f)
			| Self::Svi(f)
			| Self::Lvi(f)
			| Self::Dvi(f)
			| Self::Vb(f)
			| Self::Svb(f)
			| Self::Lvb(f)
			| Self::Dvb(f)
			| Self::Vmin(f)
			| Self::Svmin(f)
			| Self::Lvmin(f)
			| Self::Dvmin(f)
			| Self::Vmax(f)
			| Self::Svmax(f)
			| Self::Lvmax(f)
			| Self::Dvmax(f) => f,
		}
	}
}

impl FromToken for ViewportPercentage {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			let f: CSSFloat = tok.value.as_f32().unwrap().into();
			let unit = tok.value.as_atom_lower().unwrap();
			match unit {
				atom!("vw") => Ok(Self::Vw(f)),
				atom!("svw") => Ok(Self::Svw(f)),
				atom!("lvw") => Ok(Self::Lvw(f)),
				atom!("dvw") => Ok(Self::Dvw(f)),
				atom!("svh") => Ok(Self::Svh(f)),
				atom!("lvh") => Ok(Self::Lvh(f)),
				atom!("dvh") => Ok(Self::Dvh(f)),
				atom!("vi") => Ok(Self::Vi(f)),
				atom!("svi") => Ok(Self::Svi(f)),
				atom!("lvi") => Ok(Self::Lvi(f)),
				atom!("dvi") => Ok(Self::Dvi(f)),
				atom!("vb") => Ok(Self::Vb(f)),
				atom!("svb") => Ok(Self::Svb(f)),
				atom!("lvb") => Ok(Self::Lvb(f)),
				atom!("dvb") => Ok(Self::Dvb(f)),
				atom!("vmin") => Ok(Self::Vmin(f)),
				atom!("svmin") => Ok(Self::Svmin(f)),
				atom!("lvmin") => Ok(Self::Lvmin(f)),
				atom!("dvmin") => Ok(Self::Dvmin(f)),
				atom!("vmax") => Ok(Self::Vmax(f)),
				atom!("svmax") => Ok(Self::Svmax(f)),
				atom!("lvmax") => Ok(Self::Lvmax(f)),
				atom!("dvmax") => Ok(Self::Dvmax(f)),
				_ => Err(diagnostics::UnexpectedDimension(unit, tok.span))?,
			}
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
