//! `safecast` defines traits analogous to [`From`], [`Into`], [`TryFrom`], and [`TryInto`] to
//! standardize the implementation of casting between Rust types. The `can_cast_from` and
//! `can_cast_into` methods borrow the source value, allowing pattern matching without moving.

#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};

/// Conversion methods from a container type (such as an `enum`) and a target type `T`.
pub trait AsType<T>: From<T> {
    /// Borrow this instance as an instance of `T` if possible.
    fn as_type(&self) -> Option<&T>;

    /// Borrow this instance mutably as an instance of `T` if possible.
    fn as_type_mut(&mut self) -> Option<&mut T>;

    /// Convert this instance into an instance of `T` if possible.
    fn into_type(self) -> Option<T>;
}

#[macro_export]
macro_rules! as_type {
    ($c:ty, $variant:ident, $t:ty) => {
        impl From<$t> for $c {
            fn from(t: $t) -> Self {
                Self::$variant(t)
            }
        }

        impl $crate::AsType<$t> for $c {
            fn as_type(&self) -> Option<&$t> {
                match self {
                    Self::$variant(variant) => Some(variant),
                    _ => None,
                }
            }

            fn as_type_mut(&mut self) -> Option<&mut $t> {
                match self {
                    Self::$variant(variant) => Some(variant),
                    _ => None,
                }
            }

            fn into_type(self) -> Option<$t> {
                match self {
                    Self::$variant(variant) => Some(variant),
                    _ => None,
                }
            }
        }
    };
}

/// Trait for defining a cast operation from some source type `T`.
/// Analogous to [`From`].
/// The inverse of [`CastInto`].
/// Prefer implementing `CastFrom` over `CastInto` because implementing `CastFrom` automatically
/// provides an implementation of `CastInto`.
pub trait CastFrom<T> {
    /// Cast an instance of `T` into an instance of `Self`.
    fn cast_from(value: T) -> Self;
}

/// Trait for defining a cast operation to some destination type `T`.
/// Analogous to [`Into`].
/// The inverse of [`CastFrom`].
/// Prefer implementing `CastFrom` over `CastInto` because implementing `CastFrom` automatically
/// provides an implementation of `CastInto`.
pub trait CastInto<T> {
    /// Cast an instance of `Self` into an instance of `T`.
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

/// Trait for defining a cast operation when the source type cannot always be cast to the
/// destination type. Defines a `can_cast_from` method which borrows the source value, allowing
/// for pattern matching without moving the value. When `can_cast_from` returns `true`, calling
/// `opt_cast_from` *must* return `Some(...)`, otherwise `try_cast_from` may panic.
///
/// Analogous to [`TryFrom`].
/// The inverse of [`TryCastInto`].
/// Prefer implementing `TryCastFrom` over `TryCastInto` because implementing `TryCastFrom`
/// automatically provides an implementation of `TryCastInto`.
pub trait TryCastFrom<T>: Sized {
    /// Test if `value` can be cast into `Self`.
    fn can_cast_from(value: &T) -> bool;

    /// Returns `Some(Self)` if the source value can be cast into `Self`, otherwise `None`.
    fn opt_cast_from(value: T) -> Option<Self>;

    /// Returns `Ok(Self)` if the source value can be cast into `Self`, otherwise calls `on_err`.
    fn try_cast_from<Err, OnErr: FnOnce(&T) -> Err>(value: T, on_err: OnErr) -> Result<Self, Err> {
        if Self::can_cast_from(&value) {
            Ok(Self::opt_cast_from(value).unwrap())
        } else {
            Err(on_err(&value))
        }
    }
}
/// Trait for defining a cast operation when the destination type cannot always be cast from the
/// source type. Defines a `can_cast_into` method which borrows `self`, allowing for pattern
/// matching without moving `self`. If `can_cast_into` returns `true`, then calling
/// `opt_cast_into` *must* return `Some(...)`, otherwise `try_cast_into` may panic.
///
/// Analogous to [`TryFrom`].
/// The inverse of [`TryCastInto`].
/// Prefer implementing `TryCastFrom` over `TryCastInto` because implementing `TryCastFrom`
/// automatically provides an implementation of `TryCastInto`.
pub trait TryCastInto<T>: Sized {
    /// Test if `self` can be cast into `T`.
    fn can_cast_into(&self) -> bool;

    /// Returns `Some(T)` if `self` can be cast into `T`, otherwise `None`.
    fn opt_cast_into(self) -> Option<T>;

    /// Returns `Ok(T)` if `self` can be cast into `T`, otherwise calls `on_err`.
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

/// Blanket implementation of a convenience method `matches` which allows calling
/// `can_cast_from` with a type parameter. Do not implement this trait.
pub trait Match: Sized {
    /// Returns `true` if `self` can be cast into the target type `T`.
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

    #[allow(dead_code)]
    enum FooBar {
        Foo(Foo),
        Bar(Bar),
        Baz(Baz),
    }

    as_type!(FooBar, Bar, Bar);

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

    #[test]
    fn test_as_type_macro() {
        let bar = Bar { b: 0 };
        let foo_bar = FooBar::Bar(bar);
        assert_eq!(foo_bar.as_type(), Some(&bar));
    }
}
