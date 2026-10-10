use defmt::info;

use crate::keyboard::KeyName;
use crate::display::DisplayStackView;

pub struct Calculate<'a> {//<'a>{
    stack: &'a mut [f64;4],
}

impl <'a>Calculate<'a>{
    pub fn new(display_stack: &'a mut [f64;4])->Self{
        info!("creating stack");
        Self {
            stack: display_stack,
        }
    }

// Okay - figure out how to get reference to the stack here so
// that calculations can be applied.


    pub fn process_calculate_key(&mut self, key: KeyName){
        info!("process_calculate_key: {}", key);      
        match key{
            KeyName::Plus => {
                info!("In the process of the Plus key");


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