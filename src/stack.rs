use defmt::{info};

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


#[derive(Copy, Clone, Debug)]
pub struct Stack{
    array: [f64; 4],
    changed: bool,
}

impl Stack {
    pub fn new()-> Stack{
        Stack { array: [0.0; 4], changed: false }
    }

    pub fn push(&mut self, x: f64) {
        info!("Pushed {} onto stack", x);
        self.array[3] = self.array[2];
        self.array[2] = self.array[1];
        self.array[1] = self.array[0];
        self.array[0] = x;
        self.changed = true;}



    pub fn get_all(self)->(f64, f64, f64, f64){
        (self.array[0], self.array[1], self.array[2], self.array[3])
    }


    // Pops and returns bottom, x, value
    pub fn _pop(&mut self)-> f64 {
        let temp = self.array[0];
        self.array[0] = self.array[1];
        self.array[1] = self.array[2];
        self.array[2] = self.array[3];
        self.changed = true;
        return temp;
    }


    pub fn _swapxy(&mut self){
        let temp = self.array[0];
        self.array[0] = self.array[1];
        self.array[1] = temp;
        self.changed = true;
    }


    pub fn _swapx_with_new_y(&mut self, new_y: f64){
        self.array[0] = self.array[1];
        self.array[1] = new_y;
        self.changed = true;
    }


    pub fn _get_x(&mut self)->f64{
        return self.array[0];
    }
    
    pub fn _get_y(&mut self)->f64{
        return self.array[1];
    }


}