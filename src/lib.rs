/// # Function Overloading
///
/// `Function overloading` is means defining multiple functions by assigning a function with the same name to different types of `parameters` or `return type`.
pub mod overloading {
    pub struct Overloading;

    /// A trait consists of function overloading.
    ///
    /// generic `T` is the type of the function argument.
    ///
    /// a function takes only one argument. if it takes multiple arguments, use `tuple` etc.
    pub trait Foo<T> {
        /// The return type of the function.
        ///
        /// for example, if you return `Result<usize, &str>`, you can use:
        ///
        /// ```rust
        /// type Output = Result<usize, String>;
        /// ```
        type Output;
        /// the constructor of the struct (Function body).
        ///
        /// example:
        ///
        /// ```rust
        /// fn ctor(arg: usize) -> Self::Output {
        ///     if arg < 10 {
        ///        Ok(arg)
        ///     } else {
        ///        Err("too large".to_string())
        ///     }
        /// }
        /// ```
        fn ctor(arg: T) -> Self::Output;
    }

    /// ctor(usize) -> usize
    impl Foo<usize> for Overloading {
        type Output = usize;

        fn ctor(arg: usize) -> Self::Output {
            arg * 10
        }
    }

    /// ctor(String) -> String
    impl Foo<String> for Overloading {
        type Output = String;

        fn ctor(arg: String) -> Self::Output {
            arg + "!"
        }
    }

    /// Overloading functions can be conveniently used with helper functions, etc.
    pub fn foo<T>(arg: T) -> <Overloading as Foo<T>>::Output
    where
        Overloading: Foo<T>,
    {
        <Overloading as Foo<T>>::ctor(arg)
    }
}

/// # Monad
///
/// Monad is a typical functional programming languages. See [here](https://en.wikipedia.org/wiki/Monad_(functional_programming)) for more details.
pub mod monad {
    /// A simple monad implementation.
    ///
    /// It takes types `T` and `E`, which can be implemented in `Option<T>`, `Result<T, E>`, etc.
    pub trait Monad {
        type T;
        type U;

        /// for example:
        ///
        /// ```rust
        /// assert_eq!(Some(2).bind(|x| Some(x + 1)), Some(3));
        /// ```
        fn bind<F>(self, f: F) -> Self::U
        where
            F: FnOnce(Self::T) -> Self::U;
    }

    /// Monad implementation for `Option<T>`.
    impl<T> Monad for Option<T> {
        type T = T;
        type U = Option<T>;

        fn bind<F>(self, f: F) -> Self::U
        where
            F: FnOnce(Self::T) -> Self::U,
        {
            match self {
                Some(x) => f(x),
                None => None,
            }
        }
    }
}

/// # Implementing a trait for a type with macro
///
/// Implement a repeated `impl` using a `macro_rules`.
pub mod impl_macro {
    /// `Foo` trait, generic `T`
    pub trait Foo<T> {
        /// The return type of the function.
        type Output;
        /// `foo` function, `x` is a parameter of type `T`. it returns `Output`.
        fn foo(&self, x: T) -> Self::Output;
    }

    pub struct MyStruct<T>(pub T);

    /// Keywords cannot be used as identifiers, but can be decalred using `r#` prefix.
    macro_rules! r#impl {
        ($($t:ty /* `ty` is `type` */)*) => {
            $(
                impl Foo<$t> for MyStruct<$t> {
                    type Output = $t;

                    fn foo(&self, x: $t) -> Self::Output {
                        self.0 + x
                    }
                }
            )*
        };
    }

    // `{ }` can omit the semicolon.
    //
    // https://play.rust-lang.org/?gist=dee6113358f0390a3a353b3dd95e411f
    r#impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

    pub fn x() -> impl Foo<usize, Output = usize> {
        MyStruct(42)
    }
}

#[macro_export]
macro_rules! linq {
    (from $r:ident in $d:expr; select $s:expr;) => {
        $d.map(|$r| $s)
    };
    (from $r:ident in $d:expr; $(where $w:expr;)* select $s:expr;) => {
        $d.filter(|&$r| (true $(&$w)*)).map(|$r| $s)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overloading_test() {
        assert_eq!(overloading::foo(10), 100);
        assert_eq!(overloading::foo("hello".to_string()), "hello!");
    }

    #[test]
    fn monad_test() {
        use crate::monad::*;

        assert_eq!(Some(10).bind(|x| Some(x * 10)), Some(100));
        assert_eq!(None::<usize>.bind(|x| Some(x * 10)), None);

        let (mul_5, div_10) = (|x: usize| Some(x * 5), |x: usize| Some(x / 10));

        assert_eq!(Some(10).bind(mul_5).bind(div_10), Some(5));
    }

    #[test]
    fn impl_macro_test() {
        use crate::impl_macro::*;

        assert_eq!(impl_macro::x().foo(8), 50);
    }

    #[test]
    fn linq_test() {
        let result = linq!(
            from i in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter();
            where i % 2 == 0;
            select i + 10;
        );

        assert_eq!(result.collect::<Vec<i32>>(), vec![12, 14, 16, 18, 20]);
    }
}
