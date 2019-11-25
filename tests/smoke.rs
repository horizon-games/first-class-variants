mod generated {
    use first_class_variants::first_class_variants;
    #[first_class_variants(derive(Debug, PartialEq, Eq, Copy, Clone))]
    #[derive(Debug)]
    pub enum Foo {
        Bar(u8),
        Spam { ham: u16, eggs: u32 },
    }
}
mod tests {
    use crate::generated::*;
    use std::convert::TryInto;
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

        // A useful pattern for pulling things out of a Foo
        assert!(if let Ok(FooBar(x)) = maybe_bar {
            println!("{}", x);
            true
        } else {
            false
        });
    }
}
