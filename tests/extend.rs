use paste::paste;

use derive_more_more::Extend;

#[derive(Default, Extend)]
#[extend(usize)]
struct NewTypeAttrStruct(Vec<usize>);

#[derive(Default, Extend)]
struct NewTypeAttrField(#[extend(usize)] Vec<usize>);

#[derive(Default, Extend)]
struct MultiUnnamedAttrField(#[extend(usize)] Vec<usize>, bool);

#[derive(Default, Extend)]
#[extend(usize)]
struct SingleNamedAttrStruct {
    inner: Vec<usize>,
}

#[derive(Default, Extend)]
struct SingleNamedAttrField {
    #[extend(usize)]
    inner: Vec<usize>,
}

#[derive(Default, Extend)]
struct MultiNamedAttrField {
    #[extend(usize)]
    inner: Vec<usize>,
    #[allow(dead_code)]
    test: bool,
}

macro_rules! gen_case {
    ($Struct:ident {instance . $expr:expr}) => {
        paste! {
            #[test]
            fn [<test_ $Struct:camel:lower>]() {
                let mut instance = $Struct::default();
                instance.extend([0, 1, 2, 3]);
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
