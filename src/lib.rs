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

    impl Foo<usize> for Overloading {
        type Output = usize;

        fn ctor(arg: usize) -> Self::Output {
            arg * 10
        }
    }

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

/// The `linq` macro takes two patterns.
///
/// `from <ident> in <ident>; select <expr>`: `from` is the name of the variable to be iterated, `select` is the expression to be evaluated.
///
/// `from <ident> in <ident>; where <expr>; select <expr>`: `from` is the name of the variable to be iterated, `where` is the condition to be evaluated, `select` is the expression to be evaluated.
#[macro_export]
macro_rules! linq {
    (from $r:ident in $d:ident; select $s:expr;) =>
        { $d.map(|$r| $s) };
    (from $r:ident in $d:ident; $(where $w:expr;)* select $s:expr;)
        => { $d.filter(|&$r| (true $(&$w)*)).map(|$r| $s) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_test() {
        assert_eq!(overloading::foo(10), 100);
        assert_eq!(overloading::foo("hello".to_string()), "hello!");
    }

    #[test]
    fn linq_test() {
        let x = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter();

        let result = linq!(
            from i in x;
            where i % 2 == 0;
            select i + 10;
        )
        .collect::<Vec<i32>>();

        assert_eq!(result, vec![12, 14, 16, 18, 20]);
    }
}
