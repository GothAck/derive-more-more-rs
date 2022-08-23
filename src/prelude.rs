pub(crate) use crate::common::{attr::*, helpers::*, macros::*};

pub mod impls_prelude {
    pub use std::{fmt::Display, ops::Deref};

    pub use proc_macro2::{TokenStream, Span};
    pub use quote::{ToTokens, quote};
    pub use syn::{parse::{Parse, ParseStream}, Result, Error, spanned::Spanned};

    pub use crate::common::error::Errorable;

    pub(crate) use super::*;
}
