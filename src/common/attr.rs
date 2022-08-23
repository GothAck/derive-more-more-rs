use derive_more::Deref;
use syn::{ItemStruct, Field, Member};

use super::prelude::*;

#[derive(Deref)]
pub struct AttrItemStruct<A, F> {
    #[deref]
    item: ItemStruct,
    pub attr: Option<A>,
    pub fields: Vec<AttrField<F>>,
}

#[derive(Clone, Deref)]
pub struct AttrField<A> {
    #[deref]
    field: Field,
    pub attr: Option<A>,
    pub ident: Member,
}

pub trait AttrItemStructError {
    const ERR_NAME: &'static str;
}

pub trait Attr: Parse {
    const ATTR_IDENT: &'static str;

    fn from_attributes(attrs: &[syn::Attribute]) -> Result<Option<Self>> {
        attrs.iter()
            .fold(Ok(None), |mut acc, attr| {
                const ERR: &str = "Only one attribute supported";
                let span = attr.path.span();

                if attr.path.is_ident(Self::ATTR_IDENT) {
                    match &mut acc {
                        Ok(acc @ None) => {
                            acc.replace(attr.parse_args_with(Self::parse)?.into_set_span(span));
                        }
                        acc @ Ok(Some(..)) => {
                            *acc = Err(span.error(ERR))
                        }
                        Err(acc) => {
                            acc.combine(span.error(ERR))
                        }
                    }
                }
                acc
            })
    }

    fn set_span(&mut self, span: Span);

    #[inline(always)]
    fn into_set_span(mut self, span: Span) -> Self {
        self.set_span(span);
        self
    }
}

mod impls {
    use syn::Fields;

    use super::*;

    impl<A: Attr, F: Attr> AttrItemStruct<A, F> {
        pub fn fields_named(&self) -> bool {
            matches!(self.item.fields, Fields::Named(_))
        }
    }

    impl<A: Attr, F: Attr> AttrItemStruct<A, F>
    where
        Self: AttrItemStructError,
    {
        pub fn extract_common_attr<R>(&self) -> Result<(R, AttrField<F>)>
        where
            R: Clone,
            A: Attr + Deref<Target = R>,
            F: Attr + Deref<Target = R>,
            AttrField<F>: Clone,
        {
            if self.fields.is_empty() {
                return Err(self.span().error("Struct has no fields"));
            }

            match (self.fields_with_attr_one_or_none()?, &self.attr, self.fields.len()) {
                (Some((ty, ty_field)), None, _) => Ok((ty.deref().clone(), ty_field.clone())),
                (None, Some(ty), 1) => Ok((ty.deref().clone(), self.fields[0].clone())),
                (None, Some(_), _) => {
                    Err(self.span().error(format!("Struct has more than one field, move the `{}` attribute to the appropriate field", Self::ERR_NAME)))
                }
                _ => Err(self.span().error(format!("Expected `{}` attribute on struct or one field", Self::ERR_NAME))),
            }
        }

        fn fields_with_attr_one_or_none(&self) -> Result<Option<(&F, &AttrField<F>)>> {
            let (first, rest) = {
                let mut fields = self.fields.iter()
                    .filter_map(|f| f.attr.as_ref().map(|a| (a, f)))
                    .collect::<Vec<_>>();
                (fields.pop(), fields)
            };

            if let Some((_, field)) = &first {
                if rest.is_empty() {
                    Ok(first)
                } else {
                    let msg = format!("More than one field with `{}` attribute", Self::ERR_NAME);
                    let mut err = field.span().error(&msg);

                    for (_, field) in rest {
                        err.combine(field.span().error(&msg));
                    }

                    Err(err)
                }
            } else {
                Ok(None)
            }
        }
    }

    impl<A: Attr, F: Attr> Parse for AttrItemStruct<A, F> {
        fn parse(input: ParseStream) -> Result<Self> {
            let item: ItemStruct = input.parse()?;
            let attr = A::from_attributes(&item.attrs)?;

            let fields = item.fields.iter()
                .cloned()
                .enumerate()
                .map(AttrField::<F>::try_from)
                .fold(
                    Ok(Vec::new()),
                    |mut acc, item| {
                        match (&mut acc, item) {
                            (Ok(acc), Ok(item)) => {
                                acc.push(item);
                            }
                            (acc @ Ok(_), Err(item)) => {
                                *acc = Err(item);
                            }
                            (Err(_), Ok(_)) => {}
                            (Err(acc), Err(item)) => {
                                acc.combine(item);
                            }
                        }
                        acc
                    }
                )?;

            Ok(Self { item, attr, fields })
        }
    }

    impl<A: Attr> TryFrom<(usize, Field)> for AttrField<A> {
        type Error = Error;

        fn try_from((index, field): (usize, Field)) -> Result<Self> {
            let attr = A::from_attributes(&field.attrs)?;
            let ident = field.ident.clone().map_or_else(|| index.into(), From::from);

            Ok(Self { field, attr, ident })
        }
    }
}
