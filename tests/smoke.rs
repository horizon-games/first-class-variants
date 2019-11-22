use first_class_variants::first_class_variants;

#[first_class_variants]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Foo {
    Bar(u8),
    Spam { ham: u16, eggs: u32 },
}

#[test]
fn works() {
    let bar = Bar(1);
    let spam = Spam { ham: 2, eggs: 3 };

    let bar_spam: Foo = bar.into();
    match bar_spam {
        Foo::Bar(x) => assert_eq!(x, bar),
        _ => unreachable!("bar_spam isn't a Foo::Bar"),
    }
    let foo_spam: Foo = spam.into();
    match foo_spam {
        Foo::Spam(x) => assert_eq!(x, spam),
        _ => unreachable!("foo_spam isn't a Foo::Spam"),
    }
}
