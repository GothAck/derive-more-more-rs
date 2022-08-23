mod common;
mod extend;
mod from_iterator;
mod prelude;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use self::{extend::*, from_iterator::*};

#[proc_macro_derive(Extend, attributes(extend))]
pub fn extend(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as Extend).into_token_stream().into()
}

#[proc_macro_derive(FromIterator, attributes(from_iterator))]
pub fn from_iterator(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as FromIterator).into_token_stream().into()
}
