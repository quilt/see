#[cfg(feature = "generate")]
extern crate alloc;

pub mod error;
#[cfg(feature = "generate")]
pub mod list;
pub mod reflist;

type Index = u128;

pub mod number {
    pub trait Number {
        fn val() -> usize;
    }

    macro_rules! make_num {
        ($name:ident, $val:expr) => {
            #[derive(Clone)]
            pub struct $name;

            impl Number for $name {
                fn val() -> usize {
                    $val
                }
            }
        };
    }

    make_num!(U2, 2);
    make_num!(U3, 3);
    make_num!(U4, 4);
    make_num!(U5, 5);
}
