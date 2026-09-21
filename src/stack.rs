// use portable_atomic::{AtomicF64, Ordering};
// use defmt::info;
// use alloc::boxed::Box;

// use heapless::Vec;
use core::cell::RefCell;

extern crate alloc;
use alloc::rc::Rc;

const STACK_DEPTH: usize = 4;

#[derive(Clone)]
pub struct Stack {
    pub data: Rc<RefCell<[f64; 4]>>,
    last_x: Rc<RefCell<f64>>,
}

impl Stack {
    pub fn new()->Stack{
        let data = Rc::new(RefCell::new([0.0f64; STACK_DEPTH]));
        let handle = Rc::clone(&data);
        let last_x = Rc::new(RefCell::new(0.0f64));
        handle.borrow_mut()[3]=123.45;
        return Stack{
            data,
            last_x,
        }
    }
    
    pub fn set(&self, entry: usize, value: f64)
    {
        let handle = Rc::clone(&self.data);
        handle.borrow_mut()[entry] = value;
    
    }
    
    pub fn get_x(&self)->f64{
        let handle = Rc::clone(&self.data);
        handle.borrow()[0]
    }
    
 
}

pub fn main_stack(){
    let stack = Stack::new();
    stack.set(0, 234.5);
    stack.set(1, 456.7);
    stack.set(2, 987.6);
    stack.get_x();
    // for n in stack.data{
        // let handle = Rc::clone(&stack.data);
    for i in 0..4 {
        defmt::println!("{}: {}", i, stack.data.borrow()[i]);
    }
    
    
}




// From https://doc.rust-lang.org/book/ch15-01-box.html

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::stack::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     // let b = Box::new(5);
//     // info!("b= {}", b);
// }