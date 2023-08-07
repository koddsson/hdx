use hdx_lexer::Kind;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse::Parse, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataEnum, DeriveInput,
	Error, Fields, FieldsUnnamed, Ident, LitStr, Meta, Token,
};

use crate::{err, kebab};

fn kind_from_ident(ident: Ident) -> Kind {
	if ident == "Ident" {
		Kind::Ident
	} else if ident == "Number" {
		Kind::Number
	} else if ident == "String" {
		Kind::String
	} else if ident == "Function" {
		Kind::Function
	} else if ident == "Percentage" {
		Kind::Percentage
	} else if ident == "Dimension" {
		Kind::Dimension
	} else if ident == "AtKeyword" {
		Kind::AtKeyword
	} else {
		Kind::Undetermined
	}
}

#[derive(Clone, Debug)]
pub enum ParseableArg {
	FromToken,
	Kind(Kind),
	Atom(String),
}

impl Parse for ParseableArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let ident = input.parse::<Ident>()?;
		if ident == "from_token" {
			Ok(Self::FromToken)
		} else if ident == "kind" {
			input.parse::<Token![=]>()?;
			Ok(Self::Kind(kind_from_ident(input.parse::<Ident>()?)))
		} else if ident == "atom" {
			input.parse::<Token![=]>()?;
			Ok(Self::Atom(input.parse::<LitStr>()?.value()))
		} else {
			Err(Error::new(ident.span(), format!("Unrecognized Parseable arg {:?}", ident)))?
		}
	}
}

pub struct ParseableArgs {
	kind: Kind,
	from_token: bool,
	atom: Option<String>,
}

impl ParseableArgs {
	fn parse(attrs: &[Attribute]) -> Self {
		let mut ret = Self { kind: Kind::Undetermined, from_token: false, atom: None };
		if let Some(Attribute { meta: Meta::List(meta), .. }) =
			&attrs.iter().find(|a| a.path().is_ident("parseable"))
		{
			let args = meta
				.parse_args_with(Punctuated::<ParseableArg, Token![,]>::parse_terminated)
				.unwrap();
			for arg in args {
				match arg {
					ParseableArg::Kind(k) => ret.kind = k,
					ParseableArg::FromToken => ret.from_token = true,
					ParseableArg::Atom(s) => ret.atom = Some(s),
				}
			}
			return ret;
		}
		if ret.kind == Kind::Undetermined {
			ret.kind = Kind::Ident;
		}
		ret
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	match input.data {
		Data::Enum(DataEnum { variants, .. }) => {
			let mut ident_matchers = vec![];
			let mut function_matchers = vec![];
			let mut at_matchers = vec![];
			let mut string_matcher = None;
			let mut percentage_matcher = None;
			let mut number_matcher = None;
			let mut dimension_matcher = None;
			for var in variants {
				let var_ident = var.ident;
				let args = ParseableArgs::parse(&var.attrs);
				let str = LitStr::new(
					&args.atom.unwrap_or_else(|| kebab(format!("{}", var_ident))),
					var_ident.span(),
				);
				match var.fields {
					Fields::Unit => match args.kind {
						Kind::Ident => {
							ident_matchers.push(quote! {
								hdx_atom::atom!(#str) => {
									parser.advance();
									Ok(Self::#var_ident.spanned(span))
								}
							});
						}
						_ => {
							ident_matchers.push(err(
								ident.span(),
								"Parseable only matches Unit variants to Kind::Ident arms",
							));
						}
					},
					Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => match args.kind {
						Kind::Percentage => {
							if percentage_matcher.is_some() {
								percentage_matcher = Some(err(
									ident.span(),
									"Cannot have multiple fields match Kind::Percentage",
								));
							} else if unnamed.len() > 1 {
								percentage_matcher = Some(err(
									ident.span(),
									"The match arm for Kind::Percentage can only have a single unnamed value",
								));
							} else {
								let field = unnamed[0].clone().ty;
								let field_parse = if args.from_token {
									quote! {
										let val = #field::from_token(parser.cur())?;
										parser.advance();
									}
								} else {
									quote! {
										let val = #field::parse(parser)?;
									}
								};
								percentage_matcher = Some(quote! {
									hdx_lexer::Kind::Percentage => {
										use hdx_parser::{Parse, FromToken};
										#field_parse
										Ok(Self::#var_ident(val).spanned(span.until(parser.cur().span)))
									},
								});
							}
						}
						Kind::Dimension => {
							if dimension_matcher.is_some() {
								dimension_matcher = Some(err(
									ident.span(),
									"Cannot have multiple fields match Kind::Dimension",
								));
							} else if unnamed.len() > 1 {
								dimension_matcher = Some(err(
									ident.span(),
									"The match arm for Kind::Dimension can only have a single unnamed value",
								));
							} else {
								let field = unnamed[0].clone().ty;
								let field_parse = if args.from_token {
									quote! {
										let val = #field::from_token(parser.cur())?;
										parser.advance();
									}
								} else {
									quote! {
										let val = #field::parse(parser)?;
									}
								};
								dimension_matcher = Some(quote! {
									hdx_lexer::Kind::Dimension | hdx_lexer::Kind::Percentage => {
										use hdx_parser::{Parse, FromToken};
										#field_parse
										Ok(Self::#var_ident(val).spanned(span.until(parser.cur().span)))
									},
								});
							}
						}
						Kind::Number => {
							if number_matcher.is_some() {
								number_matcher = Some(err(
									ident.span(),
									"Cannot have multiple fields match Kind::Percentage",
								));
							} else if unnamed.len() > 1 {
								number_matcher = Some(err(
									ident.span(),
									"The match arm for Kind::Percentage can only have a single unnamed value",
								));
							} else {
								let field = unnamed[0].clone().ty;
								let field_parse = if args.from_token {
									quote! {
										let val = #field::from_token(parser.cur())?;
										parser.advance();
									}
								} else {
									quote! {
										let val = #field::parse(parser)?;
									}
								};
								number_matcher = Some(quote! {
									hdx_lexer::Kind::Number => {
										use hdx_parser::{Parse, FromToken};
										#field_parse
										Ok(Self::#var_ident(val).spanned(span.until(parser.cur().span)))
									},
								});
							}
						}
						Kind::String => {
							if string_matcher.is_some() {
								string_matcher = Some(err(
									ident.span(),
									"Cannot have multiple fields match Kind::String",
								));
							} else if unnamed.len() > 1 {
								string_matcher = Some(err(
									ident.span(),
									"The match arm for Kind::String can only have a single unnamed value",
								));
							} else {
								let field = unnamed[0].clone().ty;
								let field_parse = if args.from_token {
									quote! {
										let val = #field::from_token(parser.cur())?;
										parser.advance();
									}
								} else {
									quote! {
										let val = #field::parse(parser)?;
									}
								};
								string_matcher = Some(quote! {
									hdx_lexer::Kind::String => {
										use hdx_parser::{Parse, FromToken};
										#field_parse
										Ok(Self::#var_ident(val).spanned(span.until(parser.cur().span)))
									},
								});
							}
						}
						Kind::Function => {
							if unnamed.len() > 1 {
								function_matchers.push(err(
									ident.span(),
									"The match arm for a Kind::Function can only have a single unnamed value",
								));
							} else {
								let field = unnamed[0].clone().ty;
								let field_parse = if args.from_token {
									quote! {
										let val = #field::from_token(parser.cur())?;
										parser.advance();
									}
								} else {
									quote! {
										let val = #field::parse(parser)?;
									}
								};
								function_matchers.push(quote! {
									hdx_atom::atom!(#str) => {
										#field_parse
										Ok(Self::#var_ident(val).spanned(span.until(parser.cur().span)))
									}
								});
							}
						}
						Kind::AtKeyword => {
							if unnamed.len() > 1 {
								at_matchers.push(err(
									ident.span(),
									"The match arm for a Kind::AtKeyword can only have a single unnamed value",
								));
							} else {
								let field = unnamed[0].clone().ty;
								at_matchers.push(quote! {
									hdx_atom::atom!(#str) => {
										let val = #field::parse(parser)?;
										Self::#var_ident(val),
									}
								});
							}
						}
						k => {
							ident_matchers.push(err(
								ident.span(),
								&format!("Parseable cannot match Unnamed fields in a {:?} arm", k),
							));
						}
					},
					Fields::Named(_) => ident_matchers
						.push(err(var.fields.span(), "Cannot derive on Parseable on named fields")),
				}
			}
			let ident_match_arm = if ident_matchers.is_empty() {
				quote! {}
			} else {
				quote! {
					hdx_lexer::Kind::Ident => {
						let atom = parser.cur_atom_lower().unwrap();
						match atom {
							#(#ident_matchers)*
							_ => Err(hdx_parser::diagnostics::UnexpectedIdent(atom, span))?
						}
					}
				}
			};
			let function_match_arm = if function_matchers.is_empty() {
				quote! {}
			} else {
				quote! {
					hdx_lexer::Kind::Function => {
						use hdx_parser::{Parse, FromToken};
						let atom = parser.cur_atom_lower().unwrap();
						match atom {
							#(#function_matchers)*
							_ => Err(hdx_parser::diagnostics::UnexpectedFunction(atom, span))?
						}
					}
				}
			};
			let at_match_arm = if at_matchers.is_empty() {
				quote! {}
			} else {
				quote! {
					hdx_lexer::Kind::AtKeyword => {
						use hdx_parser::{Parse, FromToken};
						let atom = parser.cur_atom_lower().unwrap();
						match atom {
							#(#at_matchers)*
							_ => Err(hdx_parser::diagnostics::UnexpectedAtRule(atom, span))?
						}
					}
				}
			};
			quote! {
				#[automatically_derived]
				impl<'a> hdx_parser::Parse<'a> for #ident {
					fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<hdx_lexer::Spanned<Self>> {
						let span = parser.cur().span;
						match parser.cur().kind {
							#ident_match_arm
							#function_match_arm
							#at_match_arm
							#string_matcher
							#dimension_matcher
							#number_matcher
							#percentage_matcher
							k => Err(hdx_parser::diagnostics::Unexpected(k, span))?,
						}
					}
				}
			}
		}
		Data::Struct(_) => {
			err(ident.span(), "Cannot derive Parseable on a struct with named or no fields")
		}
		Data::Union(_) => err(ident.span(), "Cannot derive Parseable on a Union"),
	}
}
