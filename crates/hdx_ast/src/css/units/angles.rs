use hdx_atom::atom;
use hdx_derive::Writable;
use hdx_lexer::Kind;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};

use super::{AbsoluteUnit, CSSFloat};

const DEG_GRAD: f32 = 0.9;
const DEG_RAD: f32 = 57.295_78;
const DEG_TURN: f32 = 360.0;

// https://drafts.csswg.org/css-values/#angles
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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

impl<'a> Parse<'a> for Angle {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.next();
		match token.kind() {
			Kind::Dimension => match parser.parse_atom_lower(token) {
				atom!("grad") => Ok(Angle::Grad(parser.parse_number(token).into())),
				atom!("rad") => Ok(Angle::Rad(parser.parse_number(token).into())),
				atom!("turn") => Ok(Angle::Turn(parser.parse_number(token).into())),
				atom!("deg") => Ok(Angle::Deg(parser.parse_number(token).into())),
				atom => unexpected_ident!(parser, atom),
			},
			_ => unexpected!(parser, token),
		}
	}
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
