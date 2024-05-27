General-structs
===
`general-structures` - this library allows you to create structures with common fields, and those that are different can be specified individually.

## Example
``` rust
general_structs! {
    types
        Foo1 Foo2 Foo3 Foo4 Foo5

    general {
        gen_bar1: f32,
        gen_bar2: f32,
        gen_bar3: f32,
        gen_bar4: f32,
        gen_bar5: f32,
    }

    features
        Foo1 {
            spec_bar1: usize,
            spec_bar2: i32
        }
        Foo2 {
            spec_bar3: usize
        }
}
```

### Result:
``` rust
struct Foo1 {
    gen_bar1: f32,
    gen_bar2: f32,
    gen_bar3: f32,
    gen_bar4: f32,
    gen_bar5: f32,
    spec_bar1: usize,
    spec_bar2: i32,
}

struct Foo2 {
    gen_bar1: f32,
    gen_bar2: f32,
    gen_bar3: f32,
    gen_bar4: f32,
    gen_bar5: f32,
    spec_bar3: usize,
}

struct Foo3 {
    gen_bar1: f32,
    gen_bar2: f32,
    gen_bar3: f32,
    gen_bar4: f32,
    gen_bar5: f32,
}
...
```