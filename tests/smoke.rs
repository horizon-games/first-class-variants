use first_class_variants::first_class_variants;
use std::convert::TryInto;

#[first_class_variants(derive(Debug, PartialEq, Eq, Copy, Clone))]
#[derive(Debug)]
enum Foo {
    Bar(u8),
    Spam { ham: u16, eggs: u32 },
}

#[test]
fn works() {
    let bar = FooBar(1);
    let spam = FooSpam { ham: 2, eggs: 3 };

    let bar_foo: Foo = bar.into();
    match bar_foo {
        Foo::Bar(x) => assert_eq!(x, bar),
        _ => unreachable!("bar_foo isn't a Foo::Bar"),
    }
    let spam_foo: Foo = spam.into();
    match spam_foo {
        Foo::Spam(x) => assert_eq!(x, spam),
        _ => unreachable!("spam_foo isn't a Foo::Spam"),
    }

    let maybe_bar: Result<FooBar, ()> = bar_foo.try_into();
    assert_eq!(maybe_bar, Ok(bar));
}
