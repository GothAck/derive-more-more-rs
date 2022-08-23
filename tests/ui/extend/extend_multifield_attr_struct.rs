use derive_more_more::Extend;

#[derive(Default, Extend)]
#[extend(usize)]
struct Named {
    inner: Vec<usize>,
    test: bool,
}

#[derive(Default, Extend)]
#[extend(usize)]
struct Unnamed(Vec<usize>, bool);

fn main() {}
