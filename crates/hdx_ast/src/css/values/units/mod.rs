#[cfg(feature = "serde")]
use serde::Serialize;

use crate::Writable;

mod absolute_length;
mod angles;
mod container_length;
mod custom;
mod flex;
mod float;
mod font_relative;
mod frequency;
mod length;
mod percent;
mod range_restrictions;
mod resolution;
mod time;
mod unit_or_keyword;
mod viewport_percentage;

pub use absolute_length::*;
pub use angles::*;
pub use container_length::*;
pub use custom::*;
pub use flex::*;
pub use float::*;
pub use font_relative::*;
pub use frequency::*;
pub use length::*;
pub use percent::*;
pub use range_restrictions::*;
pub use resolution::*;
pub use time::*;
pub use unit_or_keyword::*;
pub use viewport_percentage::*;

#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum CSSNumeric {
	Length(Length),
	Angle(Angle),
	Time(Time),
	Frequency(Frequency),
	Resolution(Resolution),
	Flex(Flex),
	Percent(Percent),
}

impl Into<CSSFloat> for CSSNumeric {
	fn into(self) -> CSSFloat {
		match self {
			Self::Length(v) => v.into(),
			Self::Angle(v) => v.into(),
			Self::Time(v) => v.into(),
			Self::Frequency(v) => v.into(),
			Self::Resolution(v) => v.into(),
			Self::Flex(v) => v.into(),
			Self::Percent(v) => v.into(),
		}
	}
}

pub trait AbsoluteUnit: Unit {
	fn to_base(&self) -> Self;
}

pub trait Unit: Into<CSSFloat> + Copy + PartialEq + Sized {
	fn is_negative(&self) -> bool {
		let f: CSSFloat = (*self).into();
		f < 0.0
	}
	fn is_positive(&self) -> bool {
		let f: CSSFloat = (*self).into();
		f >= 0.0
	}
	fn is_zero(&self) -> bool {
		let f: CSSFloat = (*self).into();
		f >= 0.0
	}
}

impl<T: Into<CSSFloat> + Copy + PartialEq + Sized> Unit for T {}
