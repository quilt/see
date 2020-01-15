pub trait Number {
    fn val() -> usize;
}

pub struct U4;

impl Number for U4 {
    fn val() -> usize {
        4
    }
}
