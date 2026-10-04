#![no_std]
#![no_main]
// extern crate alloc;

mod calculate;
use calculate::Calculate;   

// use core::alloc::{GlobalAlloc, Layout};
use core::{cell::RefCell};
// use core::{fmt::Display};
// use core::mem::MaybeUninit;
use core::ptr::null_mut;
// use core::sync::atomic::Ordering;
use cortex_m::asm::delay;
// use cortex_m_rt::entry;
// use defmt::*;
// use defmt::{Format};

// use crate::State::EnterEntry;

//use crate::line_edit::EDIT_LENGTH;


use {defmt_rtt as _, panic_probe as _};

use defmt::info; //enables info! for debugging;

mod display;
use display::DisplayStruct;
use display::DisplayStyle;
// use display::DisplayLine;
use display_interface_spi::SPIInterface;
use display::DisplayStackView;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;

use embassy_rp::gpio::{Level, Output};
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::{SPI0};
// use embassy_rp::{Peri, PeripheralType};
// use embassy_rp::rom_data::{self, flash_reset_address_trans};
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi};// ClkPin, Config, MisoPin, MosiPin,


use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_7X13, FONT_9X18};//, ,FONT_10X20 FONT_9X18_BOLD};
// use profont::PROFONT_14_POINT;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;

use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;

mod flash_led;
use flash_led::FlashLed;

// use heapless::string::StringInner;
use heapless::String; //, format};

mod keyboard;
use keyboard::{Keyboard, KeyName, ENTER_AND_EDIT_ENTRY_MODE, WORK_IN_ENTRY_MODE};

mod line_edit;
use line_edit::LineEdit;
use line_edit::EDIT_LENGTH;

// use portable_atomic::AtomicF64;

use st7565::GraphicsPageBuffer;
use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;

// mod stack;
// use stack::Stack;
// pub struct Stack {
//     entries: [f64; 4]//;// pub data: Rc<RefCell<[f64; STACK_DEPTH]>>,
//     //    last_x: Rc<RefCell<f64>>,
// }
// impl Stack{
//     pub fn new()->Stack{
//         Stack {
//             entries: [0.0; 4],
//         }
//     }
// }

// struct StubAllocator;
// unsafe impl GlobalAlloc for StubAllocator {
//     unsafe fn alloc(&self, _layout: Layout ) -> *mut u8 {
//         null_mut()
//     }
//     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout){
//         //Stub no-op
//     }
// }

// #[global_allocator] // Dummy, but has to be here for the system to work
// static ALLOCATOR: StubAllocator = StubAllocator;


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
    // EnterEntry,
    Entry,
    // EnterCalculating,
    // LeaveEntry,
    Calculating,
}

#[embassy_executor::main]
async fn main (_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Give a quick flash on the RP2350 LED to show that the device is alive.
    let pico_led = Output::new(p.PIN_25, Level::High);
    let mut flash_led = FlashLed::new(pico_led, 20_000_000);
    flash_led.flash();

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
    let number_style = DisplayStyle::E(4);
  

    let stack_view = DisplayStackView::new(
        [0.0, 0.0, 0.0, 0.0],
    );

    let mut display = DisplayStruct::new(
        display, //: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
        reset_pin,  
        font,// MonoTextStyle<'a, BinaryColor>,
        stacknames_font, //: MonoTextStyle<'a, BinaryColor>,
        e_font, //: MonoTextStyle<'a, BinaryColor>,
        number_style,
        stack_view
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

    let mut machine_state: State = State::Entry;
    let mut calculate : Calculate = Calculate::new(); 
    display.set_number_style(DisplayStyle::E(3));
    let mut line_edit = LineEdit::new();
    let mut previous_state: State = machine_state;

    let mut skip_key = false;


    // ******************************************************************************************** //
    loop{
        delay(1_000_000); //100E6 is about once per second
        let pkey = keyboard.scan().await;
        if pkey.is_none() {continue};

        // Take a careful look at the logic that follows:
        


        if skip_key {
            info!("skip_key");
            skip_key = false;
            continue;   // goes to top of loop
        } 
        let key = pkey.unwrap();                        // setting skip_key
        info!("main: {} key pressed", key);  
        skip_key = false;
               
        match machine_state {
            State::Entry => {
                info!("Main: State::Entry");
                if WORK_IN_ENTRY_MODE.contains(key) || ENTER_AND_EDIT_ENTRY_MODE.contains(key) { // info!("------Enty ");
                    info!("Entry key: {}", key);
                    let entry_line = line_edit.process_number_keys(key);
                    info!("entry_line:");
                    if entry_line.is_some() {
                        let el = entry_line.as_ref().unwrap();
                        for c in el.chars() {
                            info!("entry_line char: {}", c);
                        }
                    }
                    display.update_stack_display(entry_line);
                } else {  // We've hit a key that takes out of entry, now we need to act
                    let x: f64 = line_edit.line.parse::<f64>().unwrap();    
                    display.stack_view.xyzt[0] = x;
                    previous_state = machine_state;                       
                    machine_state = State::Calculating;

                    info!("Going to Calculating state in main");
                    skip_key = true
                }
            },
            State::Calculating => {
                info!("State: calculating - key is: {}", key);
                skip_key = false;
                
                match key {
                    KeyName::Back => {
                        display.stack_view.xyzt[0] = 0.0;   

                        info!("In the process of Back key in calculating state");
                        // calculate.process_calculate_key(key);
                    },
                    // KeyName ::Enter => {
                    //     display.stack_view.push();
                    //     info!("In the process of the Enter key in calculating state");
                    //     // calculate.process_calculate_key(key);
                    // },
                    KeyName::Enter => {
                        let x = line_edit.line.parse::<f64>().unwrap();
                        display.stack_view.push_number(x);
                    },
                    KeyName::Plus => {
                        display.stack_view.binary_op(KeyName::Plus);
                    }
                    KeyName::Minus => {
                        display.stack_view.binary_op(KeyName::Minus);
                    }
                    KeyName::Multiply => {
                        display.stack_view.binary_op(KeyName::Multiply);
                    }
                    KeyName::Divide => {
                        display.stack_view.binary_op(KeyName::Divide);
                    }
                    _ => {info!("\t\tI don't yet know how to process {}", key)},
                }
                
                display.update_stack_display(None);
                info!("End of state calculating");

            },
        }
        previous_state = machine_state;
        info!("end of match machine_state in main");
    }
}
