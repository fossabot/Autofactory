#[macro_export]
macro_rules! make_array {
    ($n:expr, $constructor:expr) => {{
        let mut items: [_; $n] = std::mem::MaybeUninit::uninit().assume_init();
        for (i, place) in items.iter_mut().enumerate() {
            *place = $constructor(i);
        }
        items
    }};
}
