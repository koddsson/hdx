use std::hash::{Hash, Hasher};

use bitmask_enum::bitmask;
use hdx_atom::Atom;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Default)]
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum NumType {
	#[default]
	Float = 0x01,
	Signed = 0x10,
}

impl NumType {
	#[inline]
	pub fn is_int(&self) -> bool {
		self.and(NumType::Float) != NumType::Float
	}

	#[inline]
	pub fn is_float(&self) -> bool {
		self.contains(NumType::Float)
	}

	#[inline]
	pub fn is_signed(&self) -> bool {
		self.contains(NumType::Signed)
	}

	#[inline]
	pub fn is_signed_int(&self) -> bool {
		self.contains(NumType::Signed) && !self.contains(NumType::Float)
	}

	#[inline]
	pub fn signed(&self) -> NumType {
		self.or(NumType::Signed)
	}

	#[inline]
	pub fn float(&self) -> NumType {
		self.or(NumType::Float)
	}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "kind", content = "value"))]
pub enum Token {
	#[default]
	// A token yet to be built
	Undetermined,

	// <eof-token> - the end of a file (https://drafts.csswg.org/css-syntax/#typedef-eof-token)
	Eof,

	// <comment-token> (https://drafts.csswg.org/css-syntax/#comment-diagram)
	Comment(Atom),

	// <ident-token> (https://drafts.csswg.org/css-syntax/#ident-token-diagram)
	Ident(Atom),

	// <function-token> (https://drafts.csswg.org/css-syntax/#function-token-diagram)
	Function(Atom),

	// <at-keyword> https://drafts.csswg.org/css-syntax/#at-keyword-token-diagram
	AtKeyword(Atom),

	// "unrestricted" <hash-token> (https://drafts.csswg.org/css-syntax/#hash-token-diagram)
	Hash(Atom),

	// "id" <hash-token> (https://drafts.csswg.org/css-syntax/#hash-token-diagram)
	HashId(Atom),

	// <string-token> (https://drafts.csswg.org/css-syntax/#string-token-diagram)
	String(Atom, QuoteStyle),

	// <bad-string-token> (https://drafts.csswg.org/css-syntax/#typedef-bad-string-token)
	BadString,

	// <url-token> (https://drafts.csswg.org/css-syntax/#url-token-diagram)
	Url(Atom, QuoteStyle),

	// <bad-url-token> (https://drafts.csswg.org/css-syntax/#typedef-bad-url-token)
	BadUrl,

	// <delim-token> (https://drafts.csswg.org/css-syntax/#typedef-delim-token)
	Delim(char),

	// <number-token> (https://drafts.csswg.org/css-syntax/#number-token-diagram)
	Number(f32, NumType),

	// <dimension-token> (https://drafts.csswg.org/css-syntax/#dimension-token-diagram)
	Dimension(f32, Atom, NumType),

	// <whitespace-token> (https://drafts.csswg.org/css-syntax/#whitespace-token-diagram)
	Whitespace,

	// <cdo-token> (https://drafts.csswg.org/css-syntax/#CDO-token-diagram)
	Cdo,

	// <cdc-token> (https://drafts.csswg.org/css-syntax/#CDC-token-diagram)
	Cdc,

	// <colon-token> (https://drafts.csswg.org/css-syntax/#typedef-colon-token)
	Colon,

	// <semicolon-token> (https://drafts.csswg.org/css-syntax/#typedef-semicolon-token)
	Semicolon,

	// <comma-token> (https://drafts.csswg.org/css-syntax/#typedef-comma-token)
	Comma,

	// <[-token> (https://drafts.csswg.org/css-syntax/#tokendef-open-square)
	LeftSquare,

	// <]-token> (https://drafts.csswg.org/css-syntax/#tokendef-close-square)
	RightSquare,

	// <(-token> (https://drafts.csswg.org/css-syntax/#tokendef-open-paren)
	LeftParen,

	// <)-token> (https://drafts.csswg.org/css-syntax/#tokendef-close-paren)
	RightParen,

	// <{-token> (https://drafts.csswg.org/css-syntax/#tokendef-open-curly)
	LeftCurly,

	// <}-token> (https://drafts.csswg.org/css-syntax/#tokendef-close-curly)
	RightCurly,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "kind", content = "value"))]
pub enum QuoteStyle {
	// Some tokens/ast nodesthat would otherwise be strings (e.g. url(), named fonts) can have no quotes.
	None,
	Single,
	#[default]
	Double,
}

impl Token {
	#[inline]
	pub fn is_trivia(&self) -> bool {
		matches!(self, Token::Whitespace | Token::Comment(_))
	}

	#[inline]
	pub fn is_bad(&self) -> bool {
		matches!(self, Token::BadString | Token::BadUrl)
	}

	pub fn is_dashed_ident(&self) -> bool {
		match self {
			Token::Ident(value) => value.starts_with("--"),
			_ => false,
		}
	}

	#[inline]
	pub fn to_pairwise(&self) -> Option<PairWise> {
		PairWise::from_token(self)
	}
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum PairWise {
	Paren,
	Curly,
	Square,
}

impl PairWise {
	pub fn from_token(token: &Token) -> Option<Self> {
		match token {
			Token::LeftParen | Token::Function(_) => Some(Self::Paren),
			Token::LeftCurly => Some(Self::Curly),
			Token::LeftSquare => Some(Self::Square),
			Token::RightParen => Some(Self::Paren),
			Token::RightCurly => Some(Self::Curly),
			Token::RightSquare => Some(Self::Square),
			_ => None,
		}
	}

	pub fn start(&self) -> &Token {
		match self {
			Self::Paren => &Token::LeftParen,
			Self::Curly => &Token::LeftCurly,
			Self::Square => &Token::LeftSquare,
		}
	}

	pub fn end(&self) -> &Token {
		match self {
			Self::Paren => &Token::RightParen,
			Self::Curly => &Token::RightCurly,
			Self::Square => &Token::RightSquare,
		}
	}
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Hash for Token {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Token::Undetermined => {}
			Token::Eof => 0.hash(state),
			Token::Comment(a) => {
				1.hash(state);
				a.hash(state);
			}
			Token::Ident(a) => {
				2.hash(state);
				a.hash(state);
			}
			Token::Function(a) => {
				3.hash(state);
				a.hash(state);
			}
			Token::AtKeyword(a) => {
				4.hash(state);
				a.hash(state);
			}
			Token::Hash(a) => {
				5.hash(state);
				a.hash(state);
			}
			Token::HashId(a) => {
				6.hash(state);
				a.hash(state);
			}
			Token::String(a, s) => {
				7.hash(state);
				a.hash(state);
				s.hash(state);
			}
			Token::BadString => {
				8.hash(state);
			}
			Token::Url(a, s) => {
				9.hash(state);
				a.hash(state);
				s.hash(state);
			}
			Token::BadUrl => {
				10.hash(state);
			}
			Token::Delim(c) => {
				11.hash(state);
				c.hash(state);
			}
			Token::Number(f, n) => {
				12.hash(state);
				f.to_bits().hash(state);
				n.hash(state);
			}
			Token::Dimension(f, a, n) => {
				13.hash(state);
				f.to_bits().hash(state);
				a.hash(state);
				n.hash(state);
			}
			Token::Whitespace => 14.hash(state),
			Token::Cdo => 15.hash(state),
			Token::Cdc => 16.hash(state),
			Token::Colon => 17.hash(state),
			Token::Semicolon => 18.hash(state),
			Token::Comma => 19.hash(state),
			Token::LeftSquare => 20.hash(state),
			Token::RightSquare => 21.hash(state),
			Token::LeftParen => 22.hash(state),
			Token::RightParen => 23.hash(state),
			Token::LeftCurly => 24.hash(state),
			Token::RightCurly => 25.hash(state),
		}
	}
}
