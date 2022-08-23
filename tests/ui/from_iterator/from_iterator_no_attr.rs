use derive_more_more::FromIterator;

#[derive(Default, FromIterator)]
struct NamedMulti {
    inner: Vec<usize>,
    test: bool,
}

#[derive(Default, FromIterator)]
struct NamedSingle {
    inner: Vec<usize>,
}

#[derive(Default, FromIterator)]
struct UnnamedMulti(Vec<usize>, bool);

#[derive(Default, FromIterator)]
struct UnnamedSingle(Vec<usize>);

#[derive(Default, FromIterator)]
struct Unit;

fn main() {}
