#![no_std]
#![no_main]
// extern crate alloc;

mod calculate;
use calculate::Calculate;

use core::{cell::RefCell};
use cortex_m::asm::delay;
use crate::{keyboard::KeyName::Number0, stack::Stack};

use {defmt_rtt as _, panic_probe as _};

use defmt::{info, println}; //enables info! for debugging;

mod display;
use display::DisplayStruct;
use display::DisplayStyle;
use display_interface_spi::SPIInterface;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;

use embassy_rp::gpio::{Level, Output};
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::{SPI0};
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi};// ClkPin, Config, MisoPin, MosiPin,


use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_7X13, FONT_9X18};//, ,FONT_10X20 FONT_9X18_BOLD};
// use profont::PROFONT_14_POINT;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;

use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use heapless::String;
mod flash_led;
use flash_led::FlashLed;

mod keyboard;
use keyboard::{Keyboard, KeyName, ENTER_AND_EDIT_ENTRY_MODE, WORK_IN_ENTRY_MODE, BINARY_OPERATOR_KEYS};

mod eng_fmt;

mod line_edit;
use line_edit::LineEdit;
use line_edit::ResultValue;
// use line_edit::EDIT_LENGTH;

// use portable_atomic::AtomicF64;

use owo_colors::OwoColorize;

use st7565::GraphicsPageBuffer;
use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;

mod stack;
use crate::stack::STACK;
// use crate::stack::Stack;

use {defmt_rtt as _, panic_probe as _};

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Calculator"),
    embassy_rp::binary_info::rp_program_description!(
        c"A Reverse Polish Notation (RPN) calculator based on the HP42 or the HP32SII"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum State {
    Entry,
    Calculating,
}

#[embassy_executor::main]
async fn main (_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Give a quick flash on the RP2350 LED to show that the device is alive.
    let pico_led = &mut Output::new(p.PIN_25, Level::High);
    {
        let flash_led = &mut FlashLed::new(pico_led, 20_000_000);
        flash_led.flash();
    }
    pico_led.set_level(Level::Low); // Turn off the LED after flashing


    let mut state: State=State::Entry;

    let a0 = Output::new(p.PIN_27, Level::Low);   
    let display_config = spi::Config::default();

    let spi = Spi::new_blocking(p.SPI0, p.PIN_18, p.PIN_19, p.PIN_20, display_config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(p.PIN_21, Level::High), display_config);
    let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);
    let mut page_buffer = GraphicsPageBuffer::new();
    let display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, 
                SPI0, embassy_rp::spi::Blocking>, Output<'_>>, Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8> 
                = st7565::ST7565::new(display_interface, DOGL128_6)
        .into_graphics_mode(&mut page_buffer);       
    let reset_pin = Output::new(p.PIN_28, Level::Low);
    let font = MonoTextStyle::new(&FONT_9X18, BinaryColor::On);
    
    let stacknames_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
    let e_font = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let number_style = DisplayStyle::E(3);
  
    let mut display = DisplayStruct::new(
        display, //: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
        reset_pin,  
        font,// MonoTextStyle<'a, BinaryColor>,
        stacknames_font, //: MonoTextStyle<'a, BinaryColor>,
        e_font, //: MonoTextStyle<'a, BinaryColor>,
        number_style,
        // &mut stack_view
    );

    display.set_on(true);


    let mut keyboard = Keyboard::new(
        [
            Input::new(p.PIN_2, Pull::Down),
            Input::new(p.PIN_3, Pull::Down),
            Input::new(p.PIN_4, Pull::Down),
            Input::new(p.PIN_5, Pull::Down),
            Input::new(p.PIN_6, Pull::Down),
            Input::new(p.PIN_7, Pull::Down),
            Input::new(p.PIN_8, Pull::Down),
            Input::new(p.PIN_9, Pull::Down),
        ],
        [
            Output::new(p.PIN_10, Level::Low),
            Output::new(p.PIN_11, Level::Low),
            Output::new(p.PIN_12, Level::Low),
            Output::new(p.PIN_13, Level::Low),
            Output::new(p.PIN_14, Level::Low),
            Output::new(p.PIN_15, Level::Low),
        ],
    );

    // let mut _machine_state: State = State::Entry;
    // info!("before calculate");
    let mut calculate : Calculate = Calculate::new(); 
    // info!("after calculate");

    display.set_number_style(DisplayStyle::E(3));
    // info!("after calculate and set number style");

    display.update_stack_display(None);     // Put out initial stack display - all zeros, no entry line
    let mut line_edit = LineEdit::new();
    // let mut previous_state: State = machine_state;

    // let mut skip_key = false;  // Can't remember why I wanted to do this?
    // let mut editing : bool = false;
    info!("before main loop");

    // flash_led::FlashLed::new()
    
    let mut entry_mode = false;
    pico_led.set_level(Level::Low);
    delay(30_000_000);
    pico_led.set_level(Level::High);
    delay(30_000_000);
    pico_led.set_level(Level::Low);

    // ******************************************************************************************** //
    state = State::Calculating;

    
    'outer: loop{
        delay(1_000_000); //100E6 is about once per second
        let key = match keyboard.scan().await {
            Some(k) => k,
            None => continue,
        }; 
        
        if key == KeyName::OnOff {
            display.set_on(!display.is_on());
            continue;
        }

        if state != State::Entry {             
            if ENTER_AND_EDIT_ENTRY_MODE.contains(key) {
                state = State::Entry;
                pico_led.set_level(Level::High);
                line_edit.clear();
                info!(">>>>  We're going to start number entry with {}",key);
                // line_edit.process_number_keys(key);

                // display.update_stack_display(line_edit.get_entry_line().into());
            }
        }

        if state == State::Entry {
            if WORK_IN_ENTRY_MODE.contains(key) {
                info!("HERE");
                info!("Processing: {}", key);
                line_edit.process_number_keys(key);
                info!("Now HERE");
                display.update_stack_display(line_edit.get_entry_line().into());
            } else {
                info!("Leaving entry mode");
                let le = line_edit.value();
                if le.is_some(){
                    Stack::set_x(le.unwrap());
                    display.update_stack_display(None);
                }
                info!("le = {}", le);

                state = State::Calculating;
                pico_led.set_level(Level::Low);
            }
        }
        
        
        
        // else { // State is Entry
        //     if WORK_IN_ENTRY_MODE.contains(key) || ENTER_AND_EDIT_ENTRY_MODE.contains(key) {
        //         info!(">>>> We're in entry mode processing {}", key);
        //         line_edit.process_number_keys(key);
        //         line_edit.value();
        //         display.update_stack_display(line_edit.get_entry_line().into());
        //     } else {
        //         state = State::Calculating;
        //         display.update_stack_display(None);
        //         info!("Going to calculate");
        //         // Something else has come along to spoil the party
        //     }
        // }
    }
}
        












/* 


        info!("Outside inner loop, key is {}", key);

        'inner: loop {
            let mut repeat = true;

            info!("\n\nInside inner loop, key is {}", key);

            repeat = false;
            match state {
                State::Entry => {             
                    info!("***  entry mode");
                    if WORK_IN_ENTRY_MODE.contains(key) || ENTER_AND_EDIT_ENTRY_MODE.contains(key) {
                        info!(">>>>  and we're going to process the key {}",key);
                        line_edit.process_number_keys(key);

                        display.update_stack_display(line_edit.get_entry_line().into());
                        break 'inner;
                    } else if key == KeyName::Enter{
                        info!(" within Enter in entry mode");
                        let s = Stack::get_all();
                        info!("stack: x = {}, y = {}, z = {}, t = {}", s.0, s.1, s.2, s.3 );
                        Stack::push_x();
                        display.update_stack_display(None);


                    } else {
                        info!("*** leaving entry for calculating");
                        let x    = line_edit.value();  
                        info!("x is {}", x); 
                        if key == KeyName::Enter {
                            info!("enter, enter, enter/, enter, enter, enter");
                            display.update_stack_display(None);
                        }
                        Stack::set_x(x.unwrap());
                        display.update_stack_display(line_edit.get_entry_line().into());        
                        state = State::Calculating;  
                        
                    }
                },
                State::Calculating => {
                    info!("*** calculating mode");
                    if ENTER_AND_EDIT_ENTRY_MODE.contains(key) {    // take us into entry mode, and process the key
                        info!("*** turning entry mode on");
                        pico_led.set_level(Level::High);
                        info!("--   before line clear");
                        line_edit.clear();
                        repeat = true;
                        state = State::Entry;
                        info!("*** Leaving calculating for entry");
                    } else {
                        info!("Just calculating");
                        calculate.push(None);       // Pushes X into Y and keeps it in X   
                        if key == KeyName::E {
                            line_edit.process_number_keys(KeyName::Number1);    //E gets a 1 in front of it
                        } else if key == KeyName::DecimalPoint{
                            line_edit.process_number_keys(KeyName::Number0);    // A decimal point gets a 0 in front of it.
                        };
                        line_edit.process_number_keys(key);
                        display.update_stack_display(line_edit.get_entry_line().into());  
                        // break 'inner;
                    }
                }          
            }
            if repeat == false {break 'inner}
*/
        // }
 
        // info!("--------------------------------------------------------end of match machine_state loop in main");
        // let stack = Stack::get_all();   ;//.fetch_all();
        // info!("x: {}, y: {}, z: {}, t: {}", stack.0, stack.1, stack.2, stack.3);
        // info!("")
// }
    // let entry_line = line_edit.process_number_keys(key);
    // display.update_stack_display(line_edit.get_entry_line().into());



// I think that we need a case for what happens when there was no number
// entry. It mostly seems to work, but if I have a whole stack of 
// entered numbers, it doesn't manipulate them properly.


// When you hit add, that has to put the current entry line into the stack, 
// and then do the add.  So the add key has to check if there is an entry line, 
// and if so, push it onto the stack, and then do the add.  If there is no entry line, 
// then just do the add.  The same goes for all the other operator keys. 
// So the operator keys have to check if there is an entry line, and if so, push 
// it onto the stack, and then do the operation.  If there is no entry line, then 
// just do the operation.

// At the moment, the operator keys just do the operation, and don't check if 
// there is an entry line.

// Update: any key press that ends edit mode should put the value in the X 
// register on the stack then do the operation.


