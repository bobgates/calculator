// use core::cell::Cell;
// use cortex_m::interrupt::{self, Mutex};
// use cortex_m::interrupt;

// static COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));


// fn main() -> ! {

//     set_timer_1hz();
//     let mut last_state = false;
//     loop {
//         let state = read_signal_level();
//         if state && !last_state {
//             interrupt::free(|cs|
//                 COUNTER.borrow(cs).set(COUNTER.borrow(cs).get() + 1));
//         }
//         last_state = state;
//     }
// }
// #[interrupt]
// fn timer(){
//     interrupt::free(|cs| COUNTER.borrow(cs).set(0));
// }