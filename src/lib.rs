#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use proc_macro::{Diagnostic, Level, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    FnArg, ItemFn, Pat,
};

#[proc_macro_attribute]
pub fn rename_params(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = <ItemFn as Parse>::parse.parse(input).unwrap();
    for arg in func.sig.inputs.iter() {
        if let FnArg::Typed(pat) = arg {
            if let Pat::Ident(name) = pat.pat.as_ref() {
                Diagnostic::new(
                    Level::Note,
                    format!(
                        "arg({}), PatType.attrs.len = {}, PatIdent.attrs.len = {}",
                        name.ident,
                        pat.attrs.len(),
                        name.attrs.len()
                    ),
                )
                .emit();
            }
        }
    }
    quote!(#func).into()
}
