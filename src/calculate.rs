// #![no_std]
use core::panic;
use defmt::info;

use crate::keyboard::KeyName;
use crate::keyboard::OPERATOR_KEYS;
use crate::stack::Stack;

pub struct Calculate {//<'a>
    // temp_stack = 

    // stack: &'a mut [f64;4],
}



impl <'a>Calculate{

    pub fn new()->Self{
        Self{
            // stack: stack,    
        }
    }

    pub fn push(&mut self, value: Option<f64>){
        // Take a va;ue and push it onto the stack

        let mut stack  = Stack::fetch_all();
        stack.3 = stack.2;
        stack.2 = stack.1;
        stack.1 = stack.0;
        if value.is_some(){
            stack.0 = value.unwrap();
        }

        Stack::set_all(stack.0, stack.1, stack.2, stack.3, stack.4, stack.5);  
    }

    pub fn function_key(&mut self, key: KeyName){
        info!("function_key: {}", key);    
        match key{
            KeyName::Plus => {
                info!("In the process of the Plus key");
                // self.stack.0 = self.stack.1 + self.stack.0;
                // self.stack.1 = self.stack.2;
                // self.stack.2 = self.stack.3;
                // Stack::set_all(self.stack.0, self.stack.1, self.stack.2, self.stack.3, true, 0);  
            },
            _ => {info!("\t\tI don't yet know how to process {}", key)}
        }  
    }



    pub fn process_calculate_key(&mut self, key: KeyName){
        info!("process_calculate_key: {}", key);    

        let mut stack  = Stack::fetch_all();
        info!("x = {}, y = {}, z = {}, t = {}", stack.0, stack.1, stack.2, stack.3);

        if OPERATOR_KEYS.contains(key){
            let x = stack.0;
            let y = stack.1;
            let result = match key {
                KeyName::Plus => y + x,
                KeyName::Minus => y - x,
                KeyName::Multiply => y * x,
                KeyName::Divide => y / x,
                _ => panic!("process_calculate_key: unknown operator key: {:?}", key),
            }; 
            stack.0 = result;
            stack.1 = stack.2;
            stack.2 = stack.3;  
            Stack::set_all(stack.0, stack.1, stack.2, stack.3, stack.4, stack.5);  
            return;
        } else if key ==KeyName::RollDown{
            let s: f64 = stack.0;
            stack.0 = stack.1;
            stack.1 = stack.2;
            stack.2 = stack.3;
            stack.3 = s;
        }

        // } else if key == KeyName::Enter {
        //         info!("In the process of the Enter key");
        //         stack.3 = stack.2;
        //         stack.2 = stack.1;
        //         stack.1 = stack.0;
                 
        // } else if key == KeyName::XswapY {
        //         info!("In the process of XswapY");
        //         let a = x;
        //         stack.0 = y;
        //         stack.1 = x;
        //         stack.3 = stack.2;
        //         stack.2 = stack.1;
        //         stack.1 = stack.0;
        // }

        Stack::set_all(stack.0, stack.1, stack.2, stack.3, stack.4, stack.5); 

        match key{
            KeyName::Plus => {
                // binary_operator(+){

                info!("In the process of the Plus key");
                stack.0 = stack.1 + stack.0;
                stack.1 = stack.2;
                stack.2 = stack.3;
                Stack::set_all(stack.0, stack.1, stack.2, stack.3, stack.4, stack.5);  


                // if the line_edit buffer has something in it
                // convert it to a number and put it on the stack

                // if the line_edit buffer doesn't have something in
                // it, just push x, so x's value appears in x position
                // and y position.


                // On enter - convert line to number
                // Put number in stack.x
                // Push number onto stack. -- That's what happens with
                //                          enter - two copies of line 
                //                             on stack.
                // self.stack.push_x();
            },
            
            _ => {info!("\t\tI don't yet know how to process {:?}", key)}
        }  
        info!("x = {}, y = {}, z = {}, t = {}, after", stack.0, stack.1, stack.2, stack.3);
        
    }
}