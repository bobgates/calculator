//! This module manages the editing of the edit line,
//! when the calculator is in edit mode. It works exclusively with characters
//! and strings except when asked to return the value as a number

// use core::{f64, num};
// use core::fmt::Write;
// use core::{fmt, result};
use defmt::{info};//, 
// use defmt::Format;

// use heapless::Format;
use heapless::String;



// use crate::keyboard::Keyboard;
use crate::keyboard::{ENTER_AND_EDIT_ENTRY_MODE, WORK_IN_ENTRY_MODE};
use crate::keyboard::KeyName;


pub const EDIT_LENGTH: usize = 22;      // Two spare characters if there are a couple of off by 1 errors!

// use crate::stack::Stack;


use crate::State;
use crate::State::{Calculating, Entry};
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

// impl <'a>LineEdit<'a>{//<'_>{
//     pub fn new()->LineEdit<'a>{ //stack: &mut crate::stack::Stack)->LineEdit{

#[derive(Debug)]
pub struct LineEdit{
    // pub stack: &'a mut Stack,
    pub state: State,
    // pub previous_state: State, 
    pub line: String<EDIT_LENGTH>,
    // stack: crate::stack::Stack,
}

impl LineEdit{//<'_>{
    pub fn new()->LineEdit{ //stack: &mut crate::stack::Stack)->LineEdit{
        let line = String::<EDIT_LENGTH>::new();
        let state = Calculating; 
        // let previous_state = Calculating;   
        LineEdit {  state, line}
    }

    pub fn process_calculate_key(&mut self, key: KeyName){
        info!("process_calculate_key: {}", key);      
        match key{
            KeyName::Enter => {
                // self.stack.push_x();
            },
            _ => {info!("\t\tI don't yet know how to process {}", key)}
        }  
    }

    // Only called in Entry mode, so we know that the key is a number 
    // or a decimal point or E or +/-
    pub fn process_number_keys(&mut self, key: KeyName)->Option<String<EDIT_LENGTH>>{ 

        match key{
            KeyName::Enter => {
                for c in self.line.chars() {
                    info!("     pnk: in Enter: line char: {}", c);
                }
                if self.line.chars().last()==Some('E'){
                    info!("     pnk: line ends with E, so adding a 0");
                    self.line.push('0').unwrap();
                }
                let result = self.line.parse::<f64>();
                if result.is_ok() {
                    let a = result.unwrap();
                    info!("    pnk: result is ok, parsed value: {}", a);
                    return Some(self.line.clone());
                } else {
                    info!("    pnk: result is NOT ok");
                    for c in self.line.chars() {
                        info!("****** process_number_keys: line char: {}", c);
                    }
                    return None;
                }
        
            },                          //----------------------------**************************************** WORK HERE
            KeyName::Back => 
                if self.line.len()>1 {
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
        
        // let result = self.line.parse::<f64>();
        // result.ok()      

        Some(self.line.clone())
    }


    // Eats the current key and routes it to the appropriate state
    // pub fn process_key(&mut self, key: KeyName) { //-> (Option<f64>, State) {
    //     match self.state {
    //         Entry => {
    //             if WORK_IN_ENTRY_MODE.contains(key) | ENTER_AND_EDIT_ENTRY_MODE.contains(key){
    //                 self.process_number_keys(key);
    //                 info!("In entry, process_key: {}", key);
    //             } else {
    //                 self.state = Calculating;
    //                 info!("In entry, setting self.state to calculating for: {}", key);
    //                 self.process_calculate_key(key);
    //             }
    //         },
    //         Calculating => {
    //             if ENTER_AND_EDIT_ENTRY_MODE.contains(key){
    //                 self.state = Entry;
    //                 self.process_number_keys(key);
    //                 info!("In calculating, setting self.state to entry for: {}", key);
    //             } else {
    //                 info!("In calculating, process_key: {}", key);
    //                 self.process_calculate_key(key);
    //             }
    //         }
    //     }
    // }

    // Takes a key name and responds if that
    // key can work in entry mode: numbers, PLUSMINUS, etc
    pub fn works_in_entry_mode(key: KeyName)->bool{
        WORK_IN_ENTRY_MODE.contains(key) | ENTER_AND_EDIT_ENTRY_MODE.contains(key)    
    }

    // Any key that triggers a change in mode to 
    // entry. Does not include eg: PLUS_MINUS
    // because that works in entry mode but
    // works different in calculating mode
    // On entry mode, put cursor in line
    pub fn enters_entry_mode(key: KeyName)->bool{
        ENTER_AND_EDIT_ENTRY_MODE.contains(key)
    }

    // pub fn get_number(&self) -> String<22> {
    //     self.line.clone()
    // }
}
    


