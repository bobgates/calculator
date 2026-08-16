use defmt::{info};

use embassy_rp::pio::program::MovOperation::Invert;
use libm::{*};

use crate::keyboard::KeyName;
// use keyboard::KeyName::{*};

// use crate::keyboard::KeyName::{Sqrt, XswapY};

// pub enum KeyName{
//     Number0 = 0, // Setting the first to a number starts an auto-numbering system
//     Number1,
//     Number2,
//     Number3,
//     Number4,
//     Number5,    //5
//     Number6,
//     Number7,
//     Number8,
//     Number9,
//     Fn1,        //10
//     Fn2, 
//     Fn3, 
//     Fn4, 
//     Fn5,        
//     Fn6,        //15
//     SigmaPlus,
//     Invert,
//     Sqrt,
//     Log,        
//     Ln,         //20
//     Xeq,
//     Sto,
//     Rcl,
//     RollDown,   
//     Sin,        //25
//     Cos,
//     Tan,
//     Enter,
//     XswapY,     
//     PlusMinus,  //30
//     E,
//     Back,
//     Up,
//     Down,       
//     Orange,     //35
//     OnOff,
//     DecimalPoint,
//     RunStop,
//     Plus,       
//     Minus,      //40
//     Divide,
//     Multiply,
//     Error,
// }
#[derive(Copy, Clone)]


pub struct Stack{
    x: f64,
    y: f64,
    z: f64,
    t: f64,
    changed: bool,
    // numbers: Vec<f64, 18>,
    _index: usize,
}

const _NUMBERS_A: &'static [f64] = &[
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

const _NUMBERS_B: &'static [f64] = &[
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
    pub fn new()-> Stack{
    
        Stack { x: 6.234567, y: 10e2, z: 5.0e10, t: 08e-6, changed: false, _index: 0}
    }


    pub fn push(&mut self, x: f64) {
        info!("Pushed {} onto stack", x);
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
        self.x = x;
        info!("Stack is now x: {}, y: {}, z: {}, t: {}", self.x, self.y, self.z, self.t);                   //
        // self.x = entry;   /
        self.changed = true;
        // Leaves x in y and in x
    }

    // Pops and returns bottom, x, value
    pub fn _pop(&mut self)-> f64 {
        let temp = self.x;
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        self.changed = true;
        // Leaves a in a and in z
        temp
    }
    pub fn set_changed(&mut self) {
        self.changed = true;
    }
    pub fn _changed(&mut self)->bool{
        self.changed
    }
    
    pub fn fetch_values(&mut self) -> (f64, f64, f64, f64){
        (self.x, self.y, self.z, self.t)
    }

    pub fn swapxy(&mut self){
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }


    pub fn _swapx_with_new_y(&mut self, new_y: f64){
        self.x = self.y;
        self.y = new_y;
    }


    pub fn _get_x(&mut self)->f64{
        return self.x;
    }
    
    pub fn _get_y(&mut self)->f64{
        return self.y;
    }

    pub fn operate(mut self, key: KeyName){

        let mut x = self.x;
        let mut y = self.y;

        // Changes x only:
        match key {

            // SIGMA+
            KeyName::Invert => x = 1.0f64/x,
            KeyName::Sqrt => x = sqrt(x),
            KeyName::Log => x = log10(x),
            KeyName::Ln => x = log(x),
            //XEQ

            //STORE RECALL ROLLDOWN
            KeyName::Sin => x = sin(x),
            KeyName::Cos => x = cos(x),
            KeyName::Tan => x = tan(x),

            // KeyName::Enter
            KeyName::XswapY => { 
                        let temp: f64 = x;
                        x=y;
                        y=temp;},
            KeyName::PlusMinus => x=-x,
            KeyName::E => x = exp(x),
//            KeyName::Back => ------------------------------ valid only in data entry

            


            _ =>  info!("this key isn't implemented yet"),
        }
        self.x = x;
        self.y = y;


        return



    }




}