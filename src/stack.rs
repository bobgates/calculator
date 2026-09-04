use core::cell::RefCell;
use defmt::info;

const STACK_DEPTH: usize = 4;

#[derive(Clone)]
pub struct Stack {
    data: RefCell<[f64; 4]>,
    last_x: f64,
}

impl Stack {
    pub const fn new() -> Stack {
        let data = RefCell::new([1.0, 2.0, 3.0, 4.0]);
        let last_x: f64 = 0.0;
        Stack{
            data,
            last_x
        }
    }
    
    pub fn last_x(&self) -> f64 {
        self.last_x
    }
    
    pub fn push(&mut self, d: f64) {
        let mut data_ref = self.data.borrow_mut();
        self.last_x = data_ref[0];
        for i in (1..STACK_DEPTH).rev() {
            data_ref[i]= data_ref[i-1];
            // println!("{} - {}", i, data_ref[i]);
        }
        data_ref[0] = d;
    }
    
    pub fn pop(&mut self)->f64 {
        let mut data_ref = self.data.borrow_mut();
        self.last_x = data_ref[0];
        let result = data_ref[0];
        for i in (1..STACK_DEPTH).rev() {
            data_ref[i-1]= data_ref[i];
            // println!("{} - {}", i, data_ref[i]);
        }
        result
    }
    
    // Swaps x and y after having stored x
    pub fn swap_xy(&mut self){
        let mut data_ref = self.data.borrow_mut();
        self.last_x = data_ref[0];
        let temp: f64 = data_ref[0];
        data_ref[0]=data_ref[1];
        data_ref[1]=temp;
    }
    
    // It seems from the Free42 that roll_down
    // does not save the x register, but brave
    // thinks it does, so I've coded it like 
    // that
    pub fn roll_down(&mut self){
        let mut data_ref = self.data.borrow_mut();
        self.last_x = data_ref[0];
        data_ref[0] = data_ref[1];
        data_ref[1] = data_ref[2];
        data_ref[2] = data_ref[3];
        data_ref[3] = self.last_x;
    }
    
    pub fn print(&self){
        let data_ref = self.data.clone(); 
        for i in 0..STACK_DEPTH{
            info!("entry {} - {}", i, data_ref.borrow()[i]);
        }
    }
    
    pub fn get_all(&mut self) -> (f64, f64, f64, f64){
    
        let data_ref = self.data.clone(); 
        for i in 0..STACK_DEPTH{
            info!("entry {} - {}", i, data_ref.borrow()[i]);
        }
        let x = data_ref.borrow()[0];
        let y = data_ref.borrow()[1];
        let z = data_ref.borrow()[2];
        let t = data_ref.borrow()[3];
        info!("X: {}, Y: {}, Z: {}, T: {}", x,y,z,t);
        let all: (f64, f64, f64, f64) = (x, y, z, t);
        all
    }
}


// use core::cell::RefCell;
// use core::fmt;

// use defmt::{info};

// // use s::cell::RefCell;

// const STACK_DEPTH: usize = 4;

// #[derive(Clone)]
// pub struct Stack {
//     data: RefCell<[f64; 4]>,
//     last_x: f64,
// }

// impl Stack {
//     pub const fn new() -> Self {
//         let data = RefCell::new([1.0, 2.0, 3.0, 4.0]);
//         let last_x: f64 = 0.0;
//         Stack{
//             data,
//             last_x
//         }
//     }
    
//     pub fn last_x(&self) -> f64 {
//         self.last_x
//     }
    
//     pub fn push(&mut self, d: f64) {
//         let mut data_ref = self.data.borrow_mut();
//         self.last_x = data_ref[0];
//         for i in (1..STACK_DEPTH).rev() {
//             data_ref[i]= data_ref[i-1];
//             // println!("{} - {}", i, data_ref[i]);
//         }
//         data_ref[0] = d;
//     }
    
//     pub fn pop(&mut self)->f64 {
//         let mut data_ref = self.data.borrow_mut();
//         self.last_x = data_ref[0];
//         let result = data_ref[0];
//         for i in (1..STACK_DEPTH).rev() {
//             data_ref[i-1]= data_ref[i];
//             // println!("{} - {}", i, data_ref[i]);
//         }
//         result
//     }
    
//     // Swaps x and y after having stored x
//     pub fn swap_xy(&mut self){
//         let mut data_ref = self.data.borrow_mut();
//         self.last_x = data_ref[0];
//         let temp: f64 = data_ref[0];
//         data_ref[0]=data_ref[1];
//         data_ref[1]=temp;
//     }
    
//     // It seems from the Free42 that roll_down
//     // does not save the x register, but brave
//     // thinks it does, so I've coded it like 
//     // that
//     pub fn roll_down(&mut self){
//         let mut data_ref = self.data.borrow_mut();
//         self.last_x = data_ref[0];
//         data_ref[0] = data_ref[1];
//         data_ref[1] = data_ref[2];
//         data_ref[2] = data_ref[3];
//         data_ref[3] = self.last_x;
//     }
    
//     pub fn debug_print(&self){
//         let data_ref = self.data.clone(); 
//         for i in 0..STACK_DEPTH{
//             info!("entry {} - {}", i, data_ref.borrow()[i]);
//         }
//     }
// }

// // pub fn other(){
    
// // }

// // fn main(){

// //     let mut stack = Stack::new();   
    
// //     stack.print();
    
// //     // stack.print();
// //     // let binding = stack.data.borrow();
// //     // let data_ref = binding.borrow();
// //     //     // data_ref[2]=100.0;

// //     // for a in 0..STACK_DEPTH {
// //     //     println!("Stack entry {} is {}", a, data_ref[a]);
// //     // }
    
// //     stack.push(123.0);
    
    
    
// //     stack.print();
    
// //     let n = stack.pop();
    
// //     println!("n: {}", n);
// //     println!("last x: {}", stack.last_x());
    
    
// //     // let data_ref = stack.data.borrow_mut();
// //     // println!("Stack entry 3 is now {}", data_ref[3]);
    
// //     // for d in 0..data_ref.len(){
// //     //     println!("{}", data_ref[d]);
// //     // }
// // }





// // pub struct Stack<T, const N: usize> {
// //     data: [Option<T>; N],
// //     len: usize,
// // }

// // impl<T, const N: usize> Stack<T, N> {
// //     pub const fn new() -> Self {
// //         Stack {
// //             data: [const { None }; N],
// //             len: 0,
// //         }
// //     }

// //     pub fn push(&mut self, item: T) -> Result<(), T> {
// //         if self.len == N {
// //             return Err(item);
// //         }
// //         self.data[self.len] = Some(item);
// //         self.len += 1;
// //         Ok(())
// //     }

// //     pub fn pop(&mut self) -> Option<T> {
// //         if self.len == 0 {
// //             return None;
// //         }
// //         self.len -= 1;
// //         self.data[self.len].take()
// //     }
// // }

















// // const _NUMBERS_A: &'static [f64] = &[
// //     123456789.0,
// //     12345678.9,
// //     1234567.89,
// //     123456.789,
// //     12345.6789,
// //     1234.56789,
// //     123.456789,
// //     12.3456789,
// //     1.23456789,
// //     0.123456789,
// //     0.0123456789,
// //     0.00123456789,
// //     0.000123456789,
// //     0.0000123456789,
// //     0.00000123456789,
// //     0.000000123456789,
// //     0.0000000123456789,
// //     0.00000000123456789,
// //     0.0,
// // ];

// // const _NUMBERS_B: &'static [f64] = &[
// //     10000000000.1,
// //     1000000000.1,
// //     10000000.001,
// //     1000000.0001,
// //     100000.00001,
// //     10000.0000,
// //     1000.0001,
// //     100.000001,
// //     10.0000001,
// //     1.00000001,
// //     0.100000001,
// //     0.0100000001,
// //     0.00100000001,
// //     0.000100000001,
// //     0.0000100000001,
// //     0.00000100000001,
// //     0.000000100000001,
// //     0.000000010000010,
// //     0.0,
// // ];




// // const STACKSIZE : usize = 4;
// // #[derive(Copy, Clone, Debug)]
// // pub struct Stack<T, const N: usize> {
// //     array: [T; N],
// //     changed: bool,
// // }

// // impl  Stack<f64, 4> {
// //     pub const fn new()-> Self{
// //         Stack { 
// //             array: [0.0f64; 4], 
// //             changed: false 
// //         }
// //     }

// //     pub fn push(&mut self, x: f64) {
// //         info!("Pushed {} onto stack", x);
// //         self.array[3] = self.array[2];
// //         self.array[2] = self.array[1];
// //         self.array[1] = self.array[0];
// //         self.array[0] = x;
// //         self.changed = true;}



// //     pub fn get_all(self)->(f64, f64, f64, f64){
// //         (self.array[0], self.array[1], self.array[2], self.array[3])
// //     }


// //     // Pops and returns bottom, x, value
// //     pub fn _pop(&mut self)-> f64 {
// //         let temp = self.array[0];
// //         self.array[0] = self.array[1];
// //         self.array[1] = self.array[2];
// //         self.array[2] = self.array[3];
// //         self.changed = true;
// //         return temp;
// //     }


// //     pub fn _swapxy(&mut self){
// //         let temp = self.array[0];
// //         self.array[0] = self.array[1];
// //         self.array[1] = temp;
// //         self.changed = true;
// //     }


// //     pub fn _swapx_with_new_y(&mut self, new_y: f64){
// //         self.array[0] = self.array[1];
// //         self.array[1] = new_y;
// //         self.changed = true;
// //     }


// //     pub fn _get_x(&mut self)->f64{
// //         return self.array[0];
// //     }
    
// //     pub fn _get_y(&mut self)->f64{
// //         return self.array[1];
// //     }


// // }