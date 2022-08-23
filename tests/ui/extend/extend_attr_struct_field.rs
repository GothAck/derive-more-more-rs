use derive_more_more::Extend;

#[derive(Default, Extend)]
#[extend(usize)]
struct NamedMultiAttrStructField {
    #[extend(usize)]
    inner: Vec<usize>,
    test: bool,
}

#[derive(Default, Extend)]
#[extend(usize)]
struct NamedSingleAttrStructField {
    #[extend(usize)]
    inner: Vec<usize>,
}

#[derive(Default, Extend)]
#[extend(usize)]
struct UnnamedMultiAttrStructField(#[extend(usize)] Vec<usize>, bool);

#[derive(Default, Extend)]
#[extend(usize)]
struct UnnamedSingleAttrStructField(#[extend(usize)] Vec<usize>);

fn main() {}
