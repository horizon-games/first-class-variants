# first-class-variants

This crate exports a single macro - `first_class_variants::first_class_variants`.
Annotating an enum with `#[first_class_variants]` will create a first-class `struct` for each of its variants and transform the enum's variants into variants returning these `structs`.

This crate currenly does not support generics at all.
PRs are welcome!

# Example

```rust
#[first_class_variants(derive(Debug, PartialEq, Eq, Copy, Clone))]
#[derive(Debug)]
enum Foo {
    Bar(u8),
    Spam { ham: u16, eggs: u32 },
}
```

will generate an enum and 2 structs:

```rust
#[derive(Debug)]
enum Foo {
    Bar(FooBar),
    Spam(FooSpam),
}
struct FooBar(u8);
struct FooSpam { ham: u16, eggs: u32 };
```

It'll also generate an `impl From<StructName> for Foo` and an `impl TryFrom<Foo> for StructName` for each struct.

Those generated structs will be given every attribute passed into the args of `first_class_variants(...)` - e.g.

```rust
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct FooBar(u8);
```
