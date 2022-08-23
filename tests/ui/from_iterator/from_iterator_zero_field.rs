use derive_more_more::FromIterator;

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct NamedZero {}

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct UnnamedZero();

#[derive(Default, FromIterator)]
#[from_iterator(usize)]
struct UnitZero;

#[derive(Default, FromIterator)]
struct NamedZeroNoAttr {}

#[derive(Default, FromIterator)]
struct UnnamedZeroNoAttr();

#[derive(Default, FromIterator)]
struct UnitZeroNoAttr;

fn main() {}
