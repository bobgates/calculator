//! This module manages the editing of the edit line,
//! when the calculator is in edit mode. It works exclusively with characters
//! and strings except when asked to return the value as a number

// use core::{f64, num};
// use core::fmt::Write;
// use core::{fmt, result};
// use core::{error::Error, result};
use defmt::{info};//, 
// use defmt::Format;

use heapless::format;
use heapless::String;



// use crate::keyboard::Keyboard;
// use crate::keyboard::{ENTER_AND_EDIT_ENTRY_MODE, WORK_IN_ENTRY_MODE};
use crate::keyboard::KeyName;


pub const EDIT_LENGTH: usize = 22;      // Two spare characters if there are a couple of off by 1 errors!

// use crate::stack::Stack;


// use crate::State;
// use crate::State::{Calculating, Entry};
/*
    Every key press calls LineEdit process key
    LineEdit has two working states: editing or not editing.

    If it is not editing then any key in 
        ENTER_AND_EDIT_ENTRY_MODE
            puts LineEdit into Editing 
          
       which will now allow it also accept keys in:
        WORK_IN_ENTRY_MODE
        Then process with those keys to build up a number
        If any other Key arrives:
        - process keys up to now into a number, or produce
          a zero on error.
        - push the number
        - turn off ENTER_AND_EDIT_ENTRY_MODE
*/

#[derive(Debug)]
pub struct LineEdit{ 
    pub line: String<EDIT_LENGTH>

}

pub enum ResultValue{
    Float(f64),
    Str(String<EDIT_LENGTH>),
}

impl LineEdit{//<'_>{
    pub fn new()->LineEdit{ 
        let line = String::<EDIT_LENGTH>::new(); 
        LineEdit {  /*state,*/ line}
    }


    pub fn get_entry_line(&self)->String<EDIT_LENGTH>{
        return self.line.clone();
    }

    pub fn clear(&mut self){
        info!("_________________ clearing line in line.edit");
        self.line = String::<EDIT_LENGTH>::new();

    }

    // Only called in Entry mode, so we know that the key is a number 
    // or a decimal point or E or +/-, or Enter or Backspace.
    pub fn process_number_keys(&mut self, key: KeyName)->Option<ResultValue> { 
// info!("In process_number_keys***************************************************************** ");
        match key{
            KeyName::Enter => {
                info!("Enter key pressed -- in process_number_keys");

                if self.line.chars().last()==Some('E'){
                    info!("     pnk: line ends with E, so adding a 0");
                    self.line.push('0').unwrap();
                }
                let result = self.line.parse::<f64>();
         
                if result.is_ok() {
                    let x: f64 = result.unwrap();
                    info!("Result is ok, x is {}", x);
                    return Some(ResultValue::Float(x));
                } else {
                    info!("    pnk: result is NOT ok");
                    return None;
                }
        
            },        
            
            KeyName::Back => 
                if self.line.len()>1 {
                    info!("popping a character from the line");
                    self.line.pop();
                } else {                                                        // !todo else..
                    self.line.pop();
                    let _ = self.line.push('0');                               // !todo - put current format of zero into self.line
                    let _ = self.line.push('.');                               // !todo - put current format of zero into self.line
                    let _ = self.line.push('0');                               // !todo - put current format of zero into self.line
                    let _ = self.line.push('0');                               // !todo - put current format of zero into self.line
                },
            KeyName::E => if !self.line.contains('E') {                // Stops two E's being entered
                                if self.line.len()<EDIT_LENGTH{         
                                let _ = self.line.push('E');
                }
            },
            KeyName::PlusMinus =>  {       // If there's an E, change sign of what follows
                if let Some(e_pos) = self.line.find('E'){               // Then make first character - or nothing
                    info!("e_pos: {} line.len: {}", e_pos, self.line.len());     //works
                    // If there's an E, change sign of what follows
                    if e_pos+1 == self.line.len() {        // If E is last character, then put a minus after it
                        if self.line.len()<EDIT_LENGTH{
                            let _ =self.line.push('-');
                        }
                    } else if e_pos+1 < self.line.len() {  // If E is last character, then put a minus after it
                       info!("something after E, so change sign of that");
                        if self.line.as_bytes()[e_pos+1]==b'-' {  // If there's a minus after the E, remove it
                            self.line.remove(e_pos+1);
                            info!("minus after E");
                        } else {                                        // If there's no minus after the E, put one there
                            info!("add minus after E");
                            if self.line.len()<EDIT_LENGTH{
                                let _ =self.line.insert(e_pos+1,'-');
                            }
                        }
                    }

                } else { // Deal with the case of the mantissa being - or not -
                    if self.line.chars().nth(0) == Some('-'){
                        self.line.remove(0);
                        // let mut chars = self.line.chars();
                    } else if self.line.len()<EDIT_LENGTH{
                            let _ =self.line.insert(0,'-');
                        } else {
                            info!("In KeyName::PlusMinus, last line, shouldn't have got here.")
                    }
                }
            }, 

            KeyName::DecimalPoint => if self.line.find('.').is_none() {
                if self.line.len()<EDIT_LENGTH{
                            let _ =self.line.push('.');
                }
            }
    
            KeyName::Number0 => if self.line.len() < EDIT_LENGTH {self.line.push('0').unwrap()},
            KeyName::Number1 => if self.line.len() < EDIT_LENGTH {self.line.push('1').unwrap()},
            KeyName::Number2 => if self.line.len() < EDIT_LENGTH {self.line.push('2').unwrap()},
            KeyName::Number3 => if self.line.len() < EDIT_LENGTH {self.line.push('3').unwrap()},
            KeyName::Number4 => if self.line.len() < EDIT_LENGTH {self.line.push('4').unwrap()},
            KeyName::Number5 => if self.line.len() < EDIT_LENGTH {self.line.push('5').unwrap()},
            KeyName::Number6 => if self.line.len() < EDIT_LENGTH {self.line.push('6').unwrap()},
            KeyName::Number7 => if self.line.len() < EDIT_LENGTH {self.line.push('7').unwrap()},
            KeyName::Number8 => if self.line.len() < EDIT_LENGTH {self.line.push('8').unwrap()},
            KeyName::Number9 => if self.line.len() < EDIT_LENGTH {self.line.push('9').unwrap()},

            _ => (), //todo!()
        };//} else {};
        for c in self.line.chars() {
            info!("process_number_keys: line char: {}", c);
        }
        
        // let result = self.line.parse::<f64>();
        // result.ok()      

        Some(ResultValue::Str(self.line.clone()))
    }


}
    


