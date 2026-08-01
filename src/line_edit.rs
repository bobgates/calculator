//! This module manages the editing of the edit line,
//! when the calculator is in edit mode. It works exclusively with characters
//! and strings except when asked to return the value as a number

// use core::{f64, num};
use core::fmt::Write;
use core::fmt;
use defmt::{info};//, 
use defmt::Format;

// use heapless::Format;
use heapless::String;

use crate::keyboard::Keyboard;
use crate::keyboard::KeyName;
// use crate::Keyboard::
// use crate::keyboard::KEYNAME;

// const EDIT_LENGTH: usize = 20;      // Maximum length of internal buffer
const EDIT_LENGTHL: usize = 20;      // Two spare characters if there are a couple of off by 1 errors!

#[derive(Clone, Debug)]
pub struct LineEdit{
    pub editing: bool,
    line: String<EDIT_LENGTHL>,
    // stack: crate::stack::Stack,
}

impl LineEdit{
    pub fn new(/*stack: crate::stack::Stack*/)->LineEdit{
        let line = String::<EDIT_LENGTHL>::new();
        let editing = false;
        LineEdit { editing, line}//, stack }
    }

    // This eats the current key and puts the value in self.line
    pub fn process_key(&mut self, key: KeyName) {

        if Keyboard::is_number_element(key){           // Handle the display of 
            info!("is a number element: {}", key);
            self.editing = true;
            match key{
                KeyName::Enter => {self.editing = false
                
                
                },//----------------------------**************************************** WORK HERE
                KeyName::Back => if self.line.len()>1{
                    self.line.pop();
                },
                KeyName::E => if !self.line.contains('E') {
                                    if self.line.len()<EDIT_LENGTHL{             // Stops 2 E's in a number
                                    let _ = self.line.push('E');
                    }
                },
                KeyName::PlusMinus => if self.line.contains('E') {       // If we're before the E, make that +-
                    if let Some(index) = self.line.find('E'){
                        if self.line.len()>(index+1) && self.line.len()<EDIT_LENGTHL {
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
                            if self.line.len()<EDIT_LENGTHL{
                                let _ =self.line.insert(0,'-');
                                
                            } else {
                                info!("In KeyName::PlusMinus, last line, shouldn't have got here.")
                            }
                        }
                    }
                }, 

                KeyName::DecimalPoint => if self.line.find('.').is_none() {
                    if self.line.len()<EDIT_LENGTHL{
                                let _ =self.line.push('.');
                    }
                }
        
                KeyName::Number0 => if self.line.len() < EDIT_LENGTHL {self.line.push('0').unwrap()},
                KeyName::Number1 => if self.line.len() < EDIT_LENGTHL {self.line.push('1').unwrap()},
                KeyName::Number2 => if self.line.len() < EDIT_LENGTHL {self.line.push('2').unwrap()},
                KeyName::Number3 => if self.line.len() < EDIT_LENGTHL {self.line.push('3').unwrap()},
                KeyName::Number4 => if self.line.len() < EDIT_LENGTHL {self.line.push('4').unwrap()},
                KeyName::Number5 => if self.line.len() < EDIT_LENGTHL {self.line.push('5').unwrap()},
                KeyName::Number6 => if self.line.len() < EDIT_LENGTHL {self.line.push('6').unwrap()},
                KeyName::Number7 => if self.line.len() < EDIT_LENGTHL {self.line.push('7').unwrap()},
                KeyName::Number8 => if self.line.len() < EDIT_LENGTHL {self.line.push('8').unwrap()},
                KeyName::Number9 => if self.line.len() < EDIT_LENGTHL {self.line.push('9').unwrap()},
                _ => todo!()
            };
            info!("before numbers:");
            for a in self.line.chars() {
                info!("process key: ---{}", a);
            }
        }
    }
    pub fn get_line(&self) -> String<20> {
        self.line.clone()
    }
}
    


