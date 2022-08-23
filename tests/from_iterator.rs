use paste::paste;

use derive_more_more::FromIterator;

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct NewTypeAttrStruct(Vec<usize>);

#[derive(Default, FromIterator)]
struct NewTypeAttrField(#[from_iterator(usize)] Vec<usize>);

#[derive(Default, FromIterator)]
struct MultiUnnamedAttrField(#[from_iterator(usize)] Vec<usize>, bool);

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct SingleNamedAttrStruct {
    inner: Vec<usize>,
}

#[derive(Default, FromIterator)]
struct SingleNamedAttrField {
    #[from_iterator(usize)]
    inner: Vec<usize>,
}

#[derive(Default, FromIterator)]
struct MultiNamedAttrField {
    #[from_iterator(usize)]
    inner: Vec<usize>,
    #[allow(dead_code)]
    test: bool,
}

macro_rules! gen_case {
    ($Struct:ident {instance . $expr:expr}) => {
        paste! {
            #[test]
            fn [<test_ $Struct:camel:lower>]() {
                let instance: $Struct = [0, 1, 2, 3].into_iter().collect();
                assert_eq!(instance.$expr, vec![0, 1, 2, 3]);
            }
        }
    };
}

gen_case! { NewTypeAttrStruct { instance.0 }}
gen_case! { NewTypeAttrField { instance.0 }}
gen_case! { MultiUnnamedAttrField { instance.0 }}
gen_case! { SingleNamedAttrStruct { instance.inner }}
gen_case! { SingleNamedAttrField { instance.inner }}
gen_case! { MultiNamedAttrField { instance.inner }}
