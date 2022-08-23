use crate::prelude::*;

type FromIteratorItemStruct = AttrItemStruct<FromIteratorAttr, FromIteratorFieldAttr>;
type FromIteratorField = AttrField<FromIteratorFieldAttr>;

pub struct FromIterator {
    item: FromIteratorItemStruct,
    ty: TypePlusOption,
    ty_field: FromIteratorField,
}

single_item_attr!(FromIteratorAttr(TypePlusOption) ATTR_IDENT = "from_iterator");
single_item_attr!(FromIteratorFieldAttr(TypePlusOption) ATTR_IDENT = "from_iterator");

mod impls {
    use super::{impls_prelude::*, *};

    impl Parse for FromIterator {
        fn parse(input: ParseStream) -> Result<Self> {
            let item: FromIteratorItemStruct = input.parse()?;

            let (ty, ty_field) = item.extract_common_attr()?;

            Ok(Self { item, ty, ty_field })
        }
    }

    impl AttrItemStructError for FromIteratorItemStruct {
        const ERR_NAME: &'static str = "from_iterator";
    }

    impl ToTokens for FromIterator {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(self.to_token_stream_impl(false));

            if self.ty.option {
                tokens.extend(self.to_token_stream_impl(true));
            }
        }
    }

    impl FromIterator {
        fn to_token_stream_impl(&self, option: bool) -> TokenStream {
            let id_item = &self.item.ident;
            let ty = &self.ty.ty;
            let ty_field = &self.ty_field.ty;

            let (ty_from, map) = if option {
                (quote!(Option<#ty>), Some(quote! { .into_iter().filter_map(::std::convert::identity) }))
            } else {
                (ty.to_token_stream(), None)
            };

            let construct = self.to_token_stream_construct();

            quote! {
                impl FromIterator<#ty_from> for #id_item {
                    fn from_iter<T: IntoIterator<Item = #ty_from>>(iter: T) -> Self {
                        let __field = <#ty_field as FromIterator<#ty>>::from_iter(
                            iter #map,
                        );
                        #construct
                    }
                }
            }
        }

        fn to_token_stream_construct(&self) -> TokenStream {
            let id_field = &self.ty_field.ident;
            let fields_named = self.item.fields_named();

            let fields: TokenStream = self.item.fields.iter()
                .map(|f| {
                    let ident = &f.ident;

                    let value = if ident == id_field {
                        quote!(__field)
                    } else {
                        quote!(Default::default())
                    };

                    if fields_named {
                        quote!(#ident: #value,)
                    } else {
                        quote!(#value,)
                    }
                })
                .collect();

            if fields_named {
                quote!(Self { #fields })
            } else {
                quote!(Self(#fields))
            }
        }
    }
}
