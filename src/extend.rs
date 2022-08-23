use crate::prelude::*;

type ExtendItemStruct = AttrItemStruct<ExtendAttr, ExtendFieldAttr>;
type ExtendField = AttrField<ExtendFieldAttr>;

pub struct Extend {
    item: ExtendItemStruct,
    ty: TypePlusOption,
    ty_field: ExtendField,
}

single_item_attr!(ExtendAttr(TypePlusOption) ATTR_IDENT = "extend");
single_item_attr!(ExtendFieldAttr(TypePlusOption) ATTR_IDENT = "extend");

mod impls {
    use super::{impls_prelude::*, *};

    impl Parse for Extend {
        fn parse(input: ParseStream) -> Result<Self> {
            let item: ExtendItemStruct = input.parse()?;

            let (ty, ty_field) = item.extract_common_attr()?;

            Ok(Self { item, ty, ty_field })
        }
    }

    impl AttrItemStructError for ExtendItemStruct {
        const ERR_NAME: &'static str = "extend";
    }

    impl ToTokens for Extend {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(self.to_token_stream_impl(false));

            if self.ty.option {
                tokens.extend(self.to_token_stream_impl(true));
            }
        }
    }

    impl Extend {
        fn to_token_stream_impl(&self, option: bool) -> TokenStream {
            let id_item = &self.item.ident;
            let ty = &self.ty.ty;
            let ty_field = &self.ty_field.ty;
            let id_field = &self.ty_field.ident;

            let (ty_from, map) = if option {
                (quote!(Option<#ty>), Some(quote! { .into_iter().filter_map(::std::convert::identity) }))
            } else {
                (ty.to_token_stream(), None)
            };

            quote! {
                impl Extend<#ty_from> for #id_item {
                    fn extend<T: IntoIterator<Item = #ty_from>>(&mut self, iter: T) {
                        <#ty_field as Extend<#ty>>::extend(
                            &mut self.#id_field,
                            iter #map,
                        );
                    }
                }
            }
        }
    }
}
