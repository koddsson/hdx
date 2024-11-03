use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error};

use crate::def::*;

pub fn generate(defs: Def, ast: DeriveInput) -> TokenStream {
	if defs.requires_allocator_lifetime() && !ast.generics.lifetimes().any(|l| l.lifetime.ident == "a") {
		return Error::new(ast.ident.span(), "this object needs the <'a> lifetime but it didn't have it. Add it").into_compile_error();
	}
	let vis = &ast.vis;
	let attrs = &ast.attrs;
	let ident = &ast.ident;
	match &ast.data {
		Data::Enum(DataEnum { variants, .. }) => {
			if !variants.is_empty() {
				return Error::new(ident.span(), "enum must be empty").into_compile_error();
			}
			if !defs.generated_data_type().is_enum() {
				return Error::new(ident.span(), "wrong structure for this syntax, please redefine as a Struct")
					.into_compile_error();
			}
		}
		Data::Struct(DataStruct { fields, .. }) => {
			if !fields.is_empty() {
				return Error::new(ident.span(), "struct must be empty").into_compile_error();
			}
			if !defs.generated_data_type().is_struct() {
				return Error::new(ident.span(), "wrong structure for this syntax, please redefine as an Enum")
					.into_compile_error();
			}
		}
		Data::Union(_) => {
			return Error::new(ident.span(), "cannot create from_syntax on Union").into_compile_error();
		}
	}
	let def = defs.generate_definition(vis, ident);
	let peek_impl = defs.generate_peek_trait_implementation(ident);
	let parse_impl = defs.generate_parse_trait_implementation(ident);
	let writecss_impl = defs.generate_writecss_trait_implementation(ident);
	quote! {
		#(#attrs)*
		#[derive(Debug, PartialEq, Clone, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#def
		#peek_impl
		#parse_impl
		#writecss_impl
	}
}