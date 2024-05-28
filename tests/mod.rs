#![cfg(test)]
#![macro_use]

use general_structs::general_structs;

#[test]
fn generate_macros() {
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
    
    let foo = Foo1 {
        bar1: 3.0,
        bar2: 3
    };
    
    let foo2 = Foo2::<usize> {
        bar1: 3.0,
        bar2: 3,
        generic_bar: 4
    };
    
    assert_eq!(foo.bar2, 3);
    assert_eq!(foo2.generic_bar, 4);
}