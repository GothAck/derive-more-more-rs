macro_rules! single_item_attr {
    ($Attr:ident($Type:ty) ATTR_IDENT = $ATTR_IDENT:literal) => {
        #[derive(Clone)]
        struct $Attr($Type, ::proc_macro2::Span);

        ::paste::paste! {
            mod [<single_item_attr_ $Attr:snake:lower>] {
                use crate::prelude::impls_prelude::*;

                use super::*;

                impl Attr for $Attr {
                    const ATTR_IDENT: &'static str = $ATTR_IDENT;

                    fn set_span(&mut self, span: Span) {
                        self.1 = span;
                    }
                }

                impl Parse for $Attr {
                    fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                        Ok(Self(input.parse()?, Span::call_site()))
                    }
                }

                impl Spanned for $Attr {
                    fn span(&self) -> Span {
                        self.1
                    }
                }

                impl Deref for $Attr {
                    type Target = $Type;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
            }
        }
    };
}

pub(crate) use single_item_attr;
