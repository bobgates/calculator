//! This module manages the editing of the edit line,
//! when the calculator is in edit mode. It works exclusively with characters
//! and strings except when asked to return the value as a number

// use core::{f64, num};
use defmt::info;

// use heapless::format;
use heapless::String;

use crate::keyboard;
use crate::keyboard::Keyboard;
use crate::keyboard::KeyName;
// use crate::Keyboard::
// use crate::keyboard::KEYNAME;

#[derive(Clone, Debug)]
pub struct LineEdit{
    pub editing: bool,
    pub line: String<20>,
}

impl LineEdit{
    pub fn new()->LineEdit{
        let line = String::<20>::new();
        let editing = false;
        LineEdit { editing, line }
    }

    // This eats the current key and puts the value in self.line
    pub fn process_key(&mut self, key: KeyName) {

        if Keyboard::is_number_element(key){           // Handle the display of 
            info!("is a number: {}", key);
            match key{
                KeyName::Enter => {},//----------------------------**************************************** WORK HERE
                KeyName::Back => if self.line.len()>1{
                    self.line.pop();
                },
                KeyName::E => if !self.line.contains('E') {              // Stops 2 E's in a row
                                    let _ = self.line.push('E');
                },
                KeyName::PlusMinus => if self.line.contains('E'){       // If we're before the E, make that +-
                    if let Some(index) = self.line.find('E'){
                        if self.line.len()>(index+1){
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
                            let _ =self.line.insert(0,'-');
                        }
                    }
                }, 
                KeyName::DecimalPoint => if self.line.find('.').is_none() {
                    let _ = self.line.push('.');
                }
        
                KeyName::Number0 => self.line.push('0').unwrap(),
                KeyName::Number1 => self.line.push('1').unwrap(),
                KeyName::Number2 => self.line.push('2').unwrap(),
                KeyName::Number3 => self.line.push('3').unwrap(),
                KeyName::Number4 => self.line.push('4').unwrap(),
                KeyName::Number5 => self.line.push('5').unwrap(),
                KeyName::Number6 => self.line.push('6').unwrap(),
                KeyName::Number7 => self.line.push('7').unwrap(),
                KeyName::Number8 => self.line.push('8').unwrap(),
                KeyName::Number9 => self.line.push('9').unwrap(),
                _ => todo!()
                };
        }
        for d in self.line.chars(){
            info!("number: {}", d);
        }
    }
}
    


