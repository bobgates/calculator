// This file contains the Stack struct and its implementation. 
// The Stack struct is used to manage the stack of numbers in the calculator. 
// It provides methods to push and pop numbers from the stack, as well as to 
// get the current state of the stack.  

use crate::display::DisplayStyle;
// #[derive(Copy, Clone)]
use spin::Mutex;

#[derive(Copy, Clone)]
pub struct Stack(f64, f64, f64, f64, bool, usize);

pub static DISPLAY_STYLE: Mutex<DisplayStyle> = Mutex::new(DisplayStyle::E(4));

pub static STACK: Mutex< Stack> = Mutex::new(Stack(0.0, 0.0, 0.0, 0.0, false, 0));

const NUMBERS_A: &'static [f64] = &[
    123456789.0,
    12345678.9,
    1234567.89,
    123456.789,
    12345.6789,
    1234.56789,
    123.456789,
    12.3456789,
    1.23456789,
    0.123456789,
    0.0123456789,
    0.00123456789,
    0.000123456789,
    0.0000123456789,
    0.00000123456789,
    0.000000123456789,
    0.0000000123456789,
    0.00000000123456789,
    0.0,
];

const NUMBERS_B: &'static [f64] = &[
    10000000000.1,
    1000000000.1,
    10000000.001,
    1000000.0001,
    100000.00001,
    10000.0000,
    1000.0001,
    100.000001,
    10.0000001,
    1.00000001,
    0.100000001,
    0.0100000001,
    0.00100000001,
    0.000100000001,
    0.0000100000001,
    0.00000100000001,
    0.000000100000001,
    0.000000010000010,
    0.0,
];


impl Stack {
    pub fn new(x: f64, y: f64, z: f64, t: f64, changed: bool, index: usize)-> Stack{
        // let stack = STACK.lock().fetch_all();
        // stack = 
        Stack( x, y, z, t, changed, index )
     }
    
    pub fn push( x: f64) {
        let mut stack = STACK.lock().fetch_all();

        stack.3 = stack.2;
        stack.2 = stack.1;
        stack.1 = stack.0;
        stack.0 = x;
        stack.4 = true;
        // Leaves x in y and in x
    }

    // Pops and returns bottom, x, value
    pub fn pop(&mut self)-> f64 {
        let mut stack = STACK.lock().fetch_all();

        let temp = stack.0;
        stack.0 = stack.1;
        stack.1 = stack.2;
        stack.2 = stack.3;
        stack.4 = true;
        // Leaves a in a and in z
        temp
    }
    // pub fn set_changed(&mut self) { // todo! make this private and only set when a push or pop occurs
    //     stack.4 = true;
    // }
    pub fn changed(&mut self)->bool{
        let stack = STACK.lock().fetch_all();
        stack.4
    }
    
    pub fn fetch_all(&mut self) -> (f64, f64, f64, f64, bool, usize){
        let stack = STACK.lock().fetch_all();
        (stack.0, stack.1, stack.2, stack.3, stack.4, stack.5)
    }

    pub fn swapxy(&mut self){
        let mut stack = STACK.lock().fetch_all();
        let temp = stack.0;
        stack.0 = stack.1;
        stack.1 = temp;
    }


    pub fn swapx_with_new_y(&mut self, new_y: f64){
        let mut stack = STACK.lock().fetch_all();

        stack.0 = stack.1;
        stack.1 = new_y;
    }


    pub fn get_x(&mut self)->f64{        
        let stack = STACK.lock().fetch_all();
        return stack.0;
    }
    
    pub fn get_y(&mut self)->f64{
        let stack = STACK.lock().fetch_all();
        return stack.1;
    }

}