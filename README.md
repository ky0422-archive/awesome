### List of codes in this repository

-   Overriding Function
-   Monad
-   `impl` with macro
-   `linq` Macro

... more will be added soon.

# Macro

> **Note**
> 
> that this is a summary of [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html):

## Declare a macro

```rust
macro_rules! foo {
    ($a:expr) => {
        println!("a: {}", $a);
    };
    ($a:expr, $b:expr) => {
        println!("a + b: {}", $a + $b);
    };
}

foo!(1);
foo!(1, 2);
```

this is what a basic macro looks like.

`(...) => { unimplemented!(); };` is a pattern, and the code that matches this code is executed.

## Pattern

### Meta Variables

The meta variable is in the form of `$name:fragment specifier`, and is bound to `$name`.

`fragment specifier` is a specifier that tells the parser what kind of token the parameter expects.

| fragment specifier | description | examples                          |
| :----------------- | ----------- | :-------------------------------- |
| `ident`            | Identifier  | `x`                               |
| `path`             | Path        | `String::new`                     |
| `expr`             | Expression  | `1 + 2`                           |
| `ty`               | Type        | `i32`, `usize`                    |
| `pat`              | Pattern     | `_`, `foo @ 1 ..= 5`              |
| `pat_param`        | Pattern     | `1` &#124; `2`                    |
| `stmt`             | Statement   | `let x = 5;`                      |
| `block`            | Block       | `{ let x = 5; x }`                |
| `item`             | Item        | `fn foo() {}`, `struct Foo;`      |
| `meta`             | Meta        | `cfg!(unix)`                      |
| `tt`               | Token Tree  | `foo`, `+`, `5`                   |
| `lifetime`         | Lifetime    | `'a`                              |
| `vis`              | Visibility  | pub (`crate`, `self`, `super` 등) |
| `literal`          | Literal     | `1`, `"foo"`                      |

When using `expr` (or `stmt`) consecutively, you must specify it as a token such as `=>`, `,`, `;`.

This is a limitation of the parser,

`pat`: `=>`, `,`, `=`, `if`, `in`
`pat_param`: `=>`, `,`, `=`, `|`, `if`, `in`
`path`, `ty`: `=>`, `,`, `=`, `|`, `;`, `:`, `>`, `>>`, `[`, `{`, `as`, `where`, `block fragment specifier`

it also works the same as above.

### Repeat

Repeated meta variables are wrapped in `$(` ... `)` `repetition operators` and can be repeated in the same way.

`repetition operators` have `*`, `+` (not working), and `?`, where `*` takes all elements, and `?` takes 0 or 1 elements: usually ` *` is mainly used.

Example:

```rust
macro_rules! foo {
    ($( $x:expr )*) => {
        $(
            println!("{}!", $x);
        )*
    }
}

foo!(1 2 3);
```

## Macro `export`/`import`

Among the ways to export and import macros, there are `Textual` scopes and path-based scopes:

### `Textual Scope`

Macros declared in top-level modules are available in child modules:

```rust
// src/lib.rs
mod x {
    macro_rules! foo {
        () => {};
    }

    foo!();

    mod foo;
}

// foo!(); // <-- cannot find macro `foo` in this scope
```

```rust
// src/x/foo.rs

foo!();
```

### `macro_use`, `macro_export` attribute

To explain the `macro_use` and `macro_export` properties, let's look at the `foo!();` part, where an error occurs in the code described above.

because the scope of `mod x { ... }` has expired, the validity of the macro has also been lost and cannot be used.

this can be addressed by adding a `macro_use` attribute or a `macro_export` attribute:

```rust
#[macro_use]
mod x {
    macro_rules! foo {
        () => {};
    }

    foo!();

    mod foo;
}

foo!();
```

```rust
mod x {
    #[macro_export]
    macro_rules! foo {
        () => {};
    }

    foo!();

    mod foo;
}

foo!();

self::foo!();
```

the difference between the two is that `macro_export` can be used as a path-based scope:

### Path-based Scope

```rust
mod foo {
    super::foo!();
    crate::foo!();
}

mod x {
    #[macro_export]
    macro_rules! foo {
        () => {};
    }
}

foo!();
self::foo!();
```
