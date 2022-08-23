use derive_more_more::Extend;

#[derive(Default, Extend)]
struct NamedMulti {
    inner: Vec<usize>,
    test: bool,
}

#[derive(Default, Extend)]
struct NamedSingle {
    inner: Vec<usize>,
}

#[derive(Default, Extend)]
struct UnnamedMulti(Vec<usize>, bool);

#[derive(Default, Extend)]
struct UnnamedSingle(Vec<usize>);

#[derive(Default, Extend)]
struct Unit;

fn main() {}
