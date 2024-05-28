General-structs
===
`general-structures` - this library allows you to create structures with common fields, and those that are different can be specified individually.

## Example
``` rust
general_structs! {
    #[derive(Debug, Clone, Copy)]
    struct Foo1 + Foo2<T> + Foo3 {
        bar1: f32,
        pub bar2: usize,
    }

    Foo2<T> {
        generic_bar: T
    }
}
```

### Result:
``` rust
#[derive(Debug, Clone, Copy)]
struct Foo1 {
    bar1: f32,
    pub bar2: usize,
}

#[derive(Debug, Clone, Copy)]
struct Foo2<T> {
    bar1: f32,
    pub bar2: usize,
    generic_bar: T
}
    
#[derive(Debug, Clone, Copy)]
struct Foo3 {
    bar1: f32,
    pub bar2: usize,
}
```