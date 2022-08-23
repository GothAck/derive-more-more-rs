use derive_more_more::Extend;

#[derive(Default, Extend)]
struct NamedMultiAttrStructField {
    #[extend(usize)]
    inner: Vec<usize>,
    #[extend(usize)]
    test: bool,
}

#[derive(Default, Extend)]
struct UnnamedMultiAttrStructField(#[extend(usize)] Vec<usize>, #[extend(usize)] bool);

fn main() {}
