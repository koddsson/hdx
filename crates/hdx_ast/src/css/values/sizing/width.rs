use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::super::units::LengthPercentage;
use crate::{Parseable, Writable};

// https://drafts.csswg.org/css-sizing-4/#sizing-values
#[derive(Parseable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum Width {
	#[default]
	Auto, // atom!("auto")
	MinContent, // atom!("min-content")
	MaxContent, // atom!("max-content")  TODO: `intrinsic` non standard
	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	Stretch,    // atom!("stretch")  TODO: -webkit-fill-available, -moz-available
	FitContent, // atom!("fit-content")
	Contain,    // atom!("contain")

	#[parseable(kind = Dimension, from_token)]
	LengthPercentage(LengthPercentage),
	#[writable(as_function = "fit-content")]
	#[parseable(kind = Function, from_token, atom = "fit-content")]
	FitContentFunction(LengthPercentage),
}

// impl<'a> Parse<'a> for Width {
// 	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
// 		let span = parser.cur().span;
// 		match parser.cur().kind {
// 			Kind::Ident => {
// 				let ident = parser.cur_atom().unwrap();
// 				match ident.to_ascii_lowercase() {
// 					atom!("auto") => Ok(Self::Auto.spanned(span)),
// 					atom!("min-content") => Ok(Self::MinContent.spanned(span)),
// 					_ => Err(diagnostics::UnexpectedIdent(
// 						parser.cur_atom().unwrap(),
// 						parser.cur().span,
// 					))?,
// 				}
// 			}
// 			Kind::Percentage => {
// 				let value = parser.cur().value.as_f32().unwrap();
// 				parser.advance();
// 				Ok(Self::Percentage(value).spanned(span))
// 			}
// 			Kind::Number => {
// 				let value = parser.cur().value.as_f32().unwrap();
// 				parser.advance();
// 				Ok(Self::Number(value).spanned(span))
// 			}
// 			k => Err(diagnostics::Unexpected(k, parser.cur().span))?,
// 		}
// 	}
// }
