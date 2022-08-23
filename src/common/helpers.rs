use syn::Type;

use super::prelude::*;

#[derive(Clone)]
pub struct TypePlusOption {
    pub ty: Box<Type>,
    pub option: bool,
}

mod impls {
    use super::*;

    mod kw {
        use syn::custom_keyword;

        custom_keyword!(option);
    }

    impl Parse for TypePlusOption {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let ty = input.parse()?;
            let option = input.parse::<kw::option>().is_ok();

            Ok(Self { ty, option })
        }
    }
}
