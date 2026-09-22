use core::cell::RefCell;

extern crate alloc;
use alloc::rc::Rc;

const STACK_DEPTH: usize = 4;

// #[derive(Clone)]
pub struct Stack {
    pub data: Rc<RefCell<[f64; STACK_DEPTH]>>,
//    last_x: Rc<RefCell<f64>>,
}

impl Stack {
    pub fn new()->Stack{
        let data = Rc::new(RefCell::new([0.0f64; STACK_DEPTH]));
        // let handle = Rc::clone(&data);
        let _last_x = Rc::new(RefCell::new(0.0f64));
        // handle.borrow_mut()[3]=123.45;
        return Stack{
            data,
//            last_x,
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

pub fn _main_stack(){
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

