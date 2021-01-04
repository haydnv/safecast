pub trait CastFrom<T> {
    fn cast_from(value: T) -> Self;
}

pub trait CastInto<T> {
    fn cast_into(self) -> T;
}

impl<F, T: From<F>> CastFrom<F> for T {
    fn cast_from(f: F) -> T {
        T::from(f)
    }
}

impl<T, F: CastFrom<T>> CastInto<F> for T {
    fn cast_into(self) -> F {
        F::cast_from(self)
    }
}

pub trait TryCastFrom<T>: Sized {
    fn can_cast_from(value: &T) -> bool;

    fn opt_cast_from(value: T) -> Option<Self>;

    fn try_cast_from<Err, OnErr: FnOnce(&T) -> Err>(value: T, on_err: OnErr) -> Result<Self, Err> {
        if Self::can_cast_from(&value) {
            Ok(Self::opt_cast_from(value).unwrap())
        } else {
            Err(on_err(&value))
        }
    }
}

pub trait TryCastInto<T>: Sized {
    fn can_cast_into(&self) -> bool;

    fn opt_cast_into(self) -> Option<T>;

    fn try_cast_into<Err, OnErr: FnOnce(&Self) -> Err>(self, on_err: OnErr) -> Result<T, Err> {
        if self.can_cast_into() {
            Ok(self.opt_cast_into().unwrap())
        } else {
            Err(on_err(&self))
        }
    }
}

impl<F, T: CastFrom<F>> TryCastFrom<F> for T {
    fn can_cast_from(_: &F) -> bool {
        true
    }

    fn opt_cast_from(f: F) -> Option<T> {
        Some(T::cast_from(f))
    }
}

impl<F, T: TryCastFrom<F>> TryCastInto<T> for F {
    fn can_cast_into(&self) -> bool {
        T::can_cast_from(self)
    }

    fn opt_cast_into(self) -> Option<T> {
        T::opt_cast_from(self)
    }
}

pub trait Match: Sized {
    fn matches<T: TryCastFrom<Self>>(&self) -> bool {
        T::can_cast_from(self)
    }
}

impl<F> Match for F {}

#[cfg(test)]
mod tests {
    use super::*;

    struct CastError;

    struct Foo {
        a: i32,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Bar {
        b: u32,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Baz {
        bar: Bar,
    }

    impl CastFrom<Foo> for Bar {
        fn cast_from(foo: Foo) -> Self {
            Bar { b: foo.a as u32 }
        }
    }

    impl TryCastFrom<Bar> for Baz {
        fn can_cast_from(bar: &Bar) -> bool {
            bar.b == 0
        }

        fn opt_cast_from(bar: Bar) -> Option<Self> {
            if bar.b == 0 {
                Some(Self { bar })
            } else {
                None
            }
        }
    }

    #[test]
    fn test_cast() {
        let foo = Foo { a: 1 };
        assert_eq!(Bar::cast_from(foo), Bar { b: 1 })
    }

    #[test]
    fn test_matches() {
        let bar0 = Bar { b: 0 };
        let bar1 = Bar { b: 1 };
        assert!(bar0.matches::<Baz>());
        assert!(!bar1.matches::<Baz>());
    }

    #[test]
    fn test_try_cast() {
        let bar0 = Bar { b: 0 };
        let bar1 = Bar { b: 1 };

        assert_eq!(Baz::opt_cast_from(bar0), Some(Baz { bar: bar0 }));
        assert_eq!(Baz::opt_cast_from(bar1), None);

        assert!(Baz::try_cast_from(bar0, |_| CastError).is_ok());
        assert!(Baz::try_cast_from(bar1, |_| CastError).is_err());
    }
}
