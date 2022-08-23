use derive_more_more::FromIterator;

#[derive(Default, FromIterator)]
struct NamedMultiAttrStructField {
    #[from_iterator(usize)]
    inner: Vec<usize>,
    #[from_iterator(usize)]
    test: bool,
}

#[derive(Default, FromIterator)]
struct UnnamedMultiAttrStructField(#[from_iterator(usize)] Vec<usize>, #[from_iterator(usize)] bool);

fn main() {}
