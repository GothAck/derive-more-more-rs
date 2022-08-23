use derive_more_more::FromIterator;

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct NamedMultiAttrStructField {
    #[from_iterator(usize)]
    inner: Vec<usize>,
    test: bool,
}

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct NamedSingleAttrStructField {
    #[from_iterator(usize)]
    inner: Vec<usize>,
}

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct UnnamedMultiAttrStructField(#[from_iterator(usize)] Vec<usize>, bool);

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct UnnamedSingleAttrStructField(#[from_iterator(usize)] Vec<usize>);

fn main() {}
