# safecast
Rust traits to define safe casting between types.

Example usage:
```rust
struct Foo {
    a: i32,
}

struct Bar {
    b: u32,
}

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

```
