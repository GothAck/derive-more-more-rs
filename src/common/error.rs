use super::prelude::*;

pub trait Errorable {
    fn error<M: Display>(&self, msg: M) -> Error;
}

mod impls {
    use super::*;

    impl Errorable for Span {
        fn error<M: Display>(&self, msg: M) -> Error {
            Error::new(*self, msg)
        }
    }
}
