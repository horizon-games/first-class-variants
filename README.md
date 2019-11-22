# first-class-variants

This crate exports a single macro - `first_class_variants::first_class_variants`.
Annotating an enum with `#[first_class_variants]` will create a first-class `struct` for each of its variants and transform the enum's variants into variants returning these `structs`.

This crate currenly does not support generics at all.
PRs are welcome!

# Example

```rust
#[first_class_variants]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Foo {
    Bar(u8),
    Spam { ham: u16, eggs: u32 },
}
```

transforms into

```rust
enum Foo {
    Bar(Bar),
    Spam(Spam),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct FooBar(u8);
impl Into<Foo> for FooBar {
    fn into(self) -> Foo {
        Foo::Bar(self)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct FooSpam { ham: u16, eggs: u32 };
impl Into<Foo> for FooSpam {
    fn into(self) -> Foo {
        Foo::Spam(self)
    }
}
```
