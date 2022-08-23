use paste::paste;

use derive_more_more::Extend;

#[derive(Default, Extend)]
#[extend(usize option)]
struct NewTypeAttrStruct(Vec<usize>);

#[derive(Default, Extend)]
struct NewTypeAttrField(#[extend(usize option)] Vec<usize>);

#[derive(Default, Extend)]
struct MultiUnnamedAttrField(#[extend(usize option)] Vec<usize>, bool);

#[derive(Default, Extend)]
#[extend(usize option)]
struct SingleNamedAttrStruct {
    inner: Vec<usize>,
}

#[derive(Default, Extend)]
struct SingleNamedAttrField {
    #[extend(usize option)]
    inner: Vec<usize>,
}

#[derive(Default, Extend)]
struct MultiNamedAttrField {
    #[extend(usize option)]
    inner: Vec<usize>,
    #[allow(dead_code)]
    test: bool,
}

macro_rules! gen_case {
    ($Struct:ident {instance . $expr:expr}) => {
        paste! {
            #[test]
            fn [<test_ $Struct:camel:lower _option>]() {
                let mut instance = $Struct::default();
                instance.extend([0, 1, 2, 3]);
                instance.extend([Some(4), None, Some(5), None, Some(6)]);
                assert_eq!(instance.$expr, vec![0, 1, 2, 3, 4, 5, 6]);
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
