#![cfg(test)]

use general_structs::general_structs;

#[test]
fn generate_macros() {
    general_structs!(
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
    );

    let foo = Foo1 {
        gen_bar1: 0.0,
        gen_bar2: 10.0,
        gen_bar3: 3.0,
        gen_bar4: 0.0,
        gen_bar5: 2.0,
        spec_bar1: 2,
        spec_bar2: 1,
    };

    assert_eq!(foo.spec_bar1, 2);
}
