# Fundamental Types

Despite requiring explicit type annotations for function arguments, return values, struct fields, and certain constructs, Rust provides two features that alleviate the upfront planning required for type specifications:

## Type inference

Type inference allows the Rust compiler to automatically deduce the types of variables and expressions based on the context in which they appear. This means that you don't always have to explicitly specify the types of your variables and expressions, as the compiler can infer them for you. This can make writing Rust code more concise and reduce the amount of boilerplate you have to write.

```rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let sum = add(5, 6); // sum is inferred to be an i32
```

## Expressive Type System

Rust's type system is expressive and concise, enabling the use of generic types and type aliases to abstract over specific types. This flexibility allows writing generic functions and structures without tightly coupling them to specific types.

```rs
struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Self { item }
    }
}
```
