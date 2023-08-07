use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, FromToken, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::{AbsoluteLength, CSSFloat, ContainerLength, FontRelative, Percent, ViewportPercentage};
use crate::Writable;

#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Length {
	Absolute(AbsoluteLength),
	Container(ContainerLength),
	FontRelative(FontRelative),
	ViewportPercentage(ViewportPercentage),
}

impl Into<CSSFloat> for Length {
	fn into(self) -> CSSFloat {
		match self {
			Self::Absolute(v) => v.into(),
			Self::FontRelative(v) => v.into(),
			Self::CContainner(v) => v.into(),
			Self::ViewportPercentage(v) => v.into(),
		}
	}
}

#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum LengthPercentage {
	Absolute(AbsoluteLength),
	FontRelative(FontRelative),
	ViewportPercentage(ViewportPercentage),
	Percent(Percent),
}

impl Into<CSSFloat> for LengthPercentage {
	fn into(self) -> CSSFloat {
		match self {
			Self::Absolute(v) => v.into(),
			Self::FontRelative(v) => v.into(),
			Self::ViewportPercentage(v) => v.into(),
			Self::Percent(v) => v.into(),
		}
	}
}

impl FromToken for LengthPercentage {
	fn from_token(tok: &Token) -> ParserResult<Self> {
		if tok.kind == Kind::Dimension {
			if let Ok(l) = AbsoluteLength::from_token(tok) {
				Ok(Self::Absolute(l))
			} else if let Ok(l) = FontRelative::from_token(tok) {
				Ok(Self::FontRelative(l))
			} else if let Ok(l) = ViewportPercentage::from_token(tok) {
				Ok(Self::ViewportPercentage(l))
			} else {
				Err(diagnostics::Unexpected(tok.kind, tok.span))?
			}
		} else if tok.kind == Kind::Percentage {
			Ok(Self::Percent(tok.value.as_f32().unwrap().into()))
		} else {
			Err(diagnostics::Unexpected(tok.kind, tok.span))?
		}
	}
}
