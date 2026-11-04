use defmt::info;

use crate::keyboard::KeyName;
use crate::stack::STACK;
use crate::stack::Stack;

// use crate::stack::stack;



// use crate::display::DisplayStackView;

pub struct Calculate {//<'a>{
    // temp_stack = 

    // stack: &'a mut [f64;4],
}



impl <'a>Calculate{
    pub fn new()->Self{ //display_stack: &'a mut [f64;4])->Self{
        
        // info!("creating stack");
        // Self {
        //     stack: display_stack,
        // }
        Self{
            
        }
    }

// Okay - figure out how to get reference to the stack here so
// that calculations can be applied.


    pub fn process_calculate_key(&mut self, key: KeyName){
        info!("process_calculate_key: {}", key);    



        let mut stack  = Stack::fetch_all();
        info!("x = {}, y = {}, z = {}, t = {}", stack.0, stack.1, stack.2, stack.3);
        match key{
            KeyName::Plus => {
                info!("In the process of the Plus key");
                stack.0 = stack.1 + stack.0;
                stack.1 = stack.2;
                stack.2 = stack.3;



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
            _ => {info!("\t\tI don't yet know how to process {}", key)}
        }  
    }
}