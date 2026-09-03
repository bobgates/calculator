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
// use crate::Keyboard::
// use crate::keyboard::KEYNAME;

const EDIT_LENGTH: usize = 22;      // Two spare characters if there are a couple of off by 1 errors!

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
pub struct Calculate{
    // pub stack: &'a mut Stack,
    placekeeper: u32,
    // stack: crate::stack::Stack,
}
