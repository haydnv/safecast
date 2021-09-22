# safecast
Rust traits to define safe casting between types.

Example usage:
```rust
use safecast::*;

struct Foo {
    a: i32,
}

struct Bar {
    b: u16,
}

enum Baz {
    Foo(Foo),
    Bar(Bar),
}

impl CastFrom<Bar> for Foo {
    fn cast_from(bar: Bar) -> Self {
        Foo { a: bar.b as i32 }
    }
}

impl TryCastFrom<Foo> for Bar {
    fn can_cast_from(foo: &Foo) -> bool {
        foo.a >= 0 && foo.a <= u16::MAX
    }

    fn opt_cast_from(foo: Foo) -> Option<Self> {
        if foo.a >= 0 && foo.a <= u16::MAX {
            Some(Self { b: foo.a as u16 })
        } else {
            None
        }
    }
}

impl AsType<Foo> for Baz {
    fn as_type(&self) -> Option<&Foo> {
        match self {
            Self::Foo(foo) => Some(foo),
            _ => None,
        }
    }

    fn as_type_mut(&mut self) -> Option<&mut Foo> {
        match self {
            Self::Foo(foo) => Some(foo),
            _ => None,
        }
    }

    fn into_type(self) -> Option<Foo> {
        match self {
            Self::Foo(foo) => Some(foo),
            _ => None,
        }
    }
}

```
