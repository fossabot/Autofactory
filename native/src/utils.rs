#[macro_export]
macro_rules! make_array {
    ($n:expr, $constructor:expr) => {{
        let mut items: [_; $n] = std::mem::MaybeUninit::zeroed().assume_init();
        println!("{}", std::mem::size_of_val(&items));
        println!("Line 6: {:#018x}", &items as *const _ as u64);
        println!("Line 7: {:#018x}", &items[0] as *const _ as u64);
        println!("Data: {:?}", *std::mem::transmute::<_, &[u64; 24]>(&items));
        for (i, place) in items.iter_mut().enumerate() {
            println!("Building, {}", i);
            println!("Place: {:#018x}", place as *const _ as u64);
            let called = $constructor(i);
            println!("If this prints then I'm not stupid.");
            *place = called;
            println!("?!??!?!!?!?");
        }
        items
    }};
}
