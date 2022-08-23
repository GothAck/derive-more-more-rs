use derive_more_more::FromIterator;

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct Named {
    inner: Vec<usize>,
    test: bool,
}

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct Unnamed(Vec<usize>, bool);

fn main() {}
