// TODO: Test for private deps (both ways!)
// TODO: Test for type and bound enforcement!

mod lib {
    pub trait A {
        pub(self) fn foo();

        pub(crate) fn bar();
    }

    impl A for () {
        pub(super) fn foo() {}

        pub(crate) fn bar() {}
    }
}

fn main() {}
