use hdx_derive::{Atomizable, Parsable, Peekable, Writable};

pub(crate) use crate::css::types::*;
pub(crate) use crate::css::units::*;

// Re-expose stylevalues for shorthands
pub(crate) use super::{CaretAnimation, CaretColor, CaretShape, OutlineColor, OutlineStyle, OutlineWidth};

// https://drafts.csswg.org/css-ui-4/#typedef-outline-line-style
// <outline-line-style> accepts the same values as <line-style> (CSS Backgrounds 3 § 3.2 Line
// Patterns: the border-style properties) with the same meaning, except that hidden is not a legal
// outline style. In addition, the outline-style property accepts the value auto.
// <line-style> = none | hidden | dotted | dashed | solid | double | groove | ridge | inset | outset
#[derive(Atomizable, Writable, Parsable, Peekable, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum OutlineLineStyle {
	None,   // atom!("none")
	Hidden, // atom!("hidden")
	Dotted, // atom!("doted")
	Dashed, // atom!("dashed")
	Solid,  // atom!("solid")
	Double, // atom!("double")
	Groove, // atom!("groove")
	Ridge,  // atom!("ridge")
	Inset,  // atom!("inset")
	Outset, // atom!("outset")
}

// https://drafts.csswg.org/css-ui-4/#typedef-appearance-compat-auto
// These values exist for compatibility of content developed for earlier non-standard versions of this property.
// They all have the same effect as auto.
// <compat-auto> = searchfield | textarea | checkbox | radio | menulist | listbox | meter | progress-bar | button
#[derive(Atomizable, Writable, Parsable, Peekable, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum CompatAuto {
	Searchfield, // atom!("searchfield")
	Textarea,    // atom!("textarea")
	Checkbox,    // atom!("checkbox")
	Radio,       // atom!("radio")
	Menulist,    // atom!("menulist")
	Listbox,     // atom!("listbox")
	Meter,       // atom!("meter")
	ProgressBar, // atom!("progress-bar")
	Button,      // atom!("button")
}

// https://drafts.csswg.org/css-ui-4/#typedef-appearance-compat-special
// These values exist for compatibility of content developed for earlier non-standard versions of this property.
// For the purpose of this specification, they all have the same effect as auto.
// However, the host language may also take these values into account when defining the native appearance of the element.
// <compat-special> = textfield | menulist-button
#[derive(Atomizable, Writable, Parsable, Peekable, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum CompatSpecial {
	Textfield,      // atom!("textfield")
	MenulistButton, // atom!("menulist-button")
}
