//! This module manages the editing of the edit line,
//! when the calculator is in edit mode. It works exclusively with characters
//! and strings except when asked to return the value as a number

// use core::{f64, num};
use core::fmt::Write;
use core::{fmt, result};
use defmt::{info};//, 
use defmt::Format;

// use heapless::Format;
use heapless::String;

use crate::keyboard::Keyboard;
use crate::keyboard::{ENTER_AND_EDIT_ENTRY_MODE, WORK_IN_ENTRY_MODE};
use crate::keyboard::KeyName;
// use crate::Keyboard::
// use crate::keyboard::KEYNAME;

// const EDIT_LENGTH: usize = 20;      // Maximum length of internal buffer
const EDIT_LENGTH: usize = 20;      // Two spare characters if there are a couple of off by 1 errors!

#[derive(Clone, Debug)]
pub struct LineEdit{
    pub editing: bool,
    pub line: String<EDIT_LENGTH>,
    // stack: crate::stack::Stack,
}

/*

    Every key press calls LineEdit process key
    LineEdit has two states: editing or not editing.

    If it is not editing then any key in 
        ENTER_AND_EDIT_ENTRY_MODE
            puts it into edit and it will now also accept:
        WORK_IN_ENTRY_MODE
        Then process with those keys to build up a number
        If any other Key arrives:
        - process keys up to now into a number, or produce
          a zero on error.
        - push the number
        - turn off ENTER_AND_EDIT_ENTRY_MODE

*/



impl LineEdit{
    pub fn new( /*stack: crate::stack::Stack*/)->LineEdit{
        let line = String::<EDIT_LENGTH>::new();
        let editing = false;
        LineEdit { editing, line}//, stack }
    }

    pub fn start_editing(&mut self){
        self.editing = true;
    }

    pub fn stop_editing(&mut self){
        self.editing = false;
    }

    pub fn process_number_keys(&mut self, key: KeyName)->Option<f64> {  

        match key{
            KeyName::Enter => {
                if self.editing {
                    for c in self.line.chars() {
                        info!("process_number_keys in Enter: line char: {}", c);
                    }

                    let result = self.line.parse::<f64>();
                    if result.is_ok() {
                        let a = result.unwrap();
                        info!("    result is ok, parsed value: {}", a);
                        // info!("process_number_keys: set editing to FALSE");
                        self.stop_editing();
                        return Some(a);
                    } else {
                        info!("    result is not ok");
                        for c in self.line.chars() {
                            info!("****** process_number_keys: line char: {}", c);
                        }
                        // info!("process_number_keys: parse failed, still editing");
                    // } */
                    }
                    return None;
                } else {
                    info!("process_number_keys: set editing to TRUE");
                    self.start_editing();                                            // !todo this is wrong...
                }
            },                          //----------------------------**************************************** WORK HERE
            KeyName::Back => 
                if self.line.len()>1 && self.editing{
                    info!("popping a character from the line");
                    self.line.pop();
                } else {                                                        // !todo else..
                    self.line.pop();
                    self.line.push('0').unwrap();                               // !todo - put current format of zero into self.line
                },
            KeyName::E => if !self.line.contains('E') {                // Stops two E's being entered
                                if self.line.len()<EDIT_LENGTH{         
                                let _ = self.line.push('E');
                }
            },
            KeyName::PlusMinus => if self.line.contains('E') {       // If we're before the E, make that +-
                if let Some(index) = self.line.find('E'){
                    if self.line.len()>(index+1) && self.line.len()<EDIT_LENGTH {
                        if self.line.as_bytes()[index+1]==b'-'{
                            self.line.remove(index+1);
                        } else {
                            if self.line.len()==(index+1){
                                let _ =self.line.insert(index+1, '-');
                            }
                        }
                    } else {                        // No E yet, so put minus at begining of number
                        if self.line.len()==(index+1){
                            let _ =self.line.push_str("-");
                        }
                    }
                } else { // Deal with the case of the mantissa being - or not -
                    if self.line.chars().nth(0) == Some('-'){
                        let _a: String<20> = self.line.chars().skip(1).collect();
                    } else {
                        if self.line.len()<EDIT_LENGTH{
                            let _ =self.line.insert(0,'-');
                            
                        } else {
                            info!("In KeyName::PlusMinus, last line, shouldn't have got here.")
                        }
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
        
        let result = self.line.parse::<f64>();
        result.ok()      
    }


    // Eats the current key and routes it to numbers or
    // calcs
    pub fn process_key(&mut self, key: KeyName) -> Option<f64> {

        if self.editing{
            if !Self::works_in_entry_mode(key){
                self.editing = false;
                info!("self.editing set false XXXXXXX");
            }
        } else {
            if Keyboard::enters_entry_mode(key){
                self.editing = true;
                info!("self.editing set to true!!!!!!!!");
            }
        };
        if self.editing{
            info!("is a number key ooooooooooooooooooooo ");
            self.process_number_keys(key)
        } else {
            //Process command
            info!("not a number key - to be implemented ");
            None
        }
    }

    pub fn is_number_element(key: KeyName)->bool{
       ENTER_AND_EDIT_ENTRY_MODE.contains(key)|WORK_IN_ENTRY_MODE.contains(key)
    }

    pub fn works_in_entry_mode(key: KeyName)->bool{
        WORK_IN_ENTRY_MODE.contains(key)        
    }

    pub fn get_number(&self) -> String<20> {
        self.line.clone()
    }
}
    


