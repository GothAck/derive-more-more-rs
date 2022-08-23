use derive_more_more::Extend;

#[derive(Default, Extend)]
#[extend(usize)]
struct NamedZero {}

#[derive(Default, Extend)]
#[extend(usize)]
struct UnnamedZero();

#[derive(Default, Extend)]
#[extend(usize)]
struct UnitZero;

#[derive(Default, Extend)]
struct NamedZeroNoAttr {}

#[derive(Default, Extend)]
struct UnnamedZeroNoAttr();

#[derive(Default, Extend)]
struct UnitZeroNoAttr;

fn main() {}
