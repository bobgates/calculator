#![no_std]
#![no_main]
extern crate alloc;
// #![no_main]

// mod alloc;
// use alloc::SimpleAllocator;

// #![feature(default_alloc_error_handler)]

// extern crate alloc;
// use alloc::alloc::
// use alloc::vec::Vec;

// use no_std_compat2::alloc::Allocator;


mod calculate;
use calculate::Calculate;   

use core::alloc::{GlobalAlloc, Layout};
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
use display::StackView;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;

use embassy_rp::gpio::{Level, Output};
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::{SPI0};
// use embassy_rp::{Peri, PeripheralType};
// use embassy_rp::rom_data::{self, flash_reset_address_trans};
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi};// ClkPin, Config, MisoPin, MosiPin,


use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_7X13, FONT_9X18};//, ,FONT_10X20 FONT_9X18_BOLD};
// use profont::PROFONT_14_POINT;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;

mod flash_led;
use flash_led::FlashLed;

// use heapless::string::StringInner;
use heapless::String; //, format};

mod keyboard;
use keyboard::Keyboard;//, KeyName};
use keyboard::{ENTER_AND_EDIT_ENTRY_MODE, WORK_IN_ENTRY_MODE};

mod line_edit;
use line_edit::LineEdit;
use line_edit::EDIT_LENGTH;

// use portable_atomic::AtomicF64;

use st7565::GraphicsPageBuffer;
use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;

mod stack;
use stack::Stack;



struct StubAllocator;
unsafe impl GlobalAlloc for StubAllocator {
    unsafe fn alloc(&self, _layout: Layout ) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout){
        //Stub no-op
    }
}

#[global_allocator]
static ALLOCATOR: StubAllocator = StubAllocator;




// use defmt::{Format};
use {defmt_rtt as _, panic_probe as _};

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
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


// use alloc::{GlobalAlloc, Layout};


// No custom global allocator is configured here.
// A valid allocator definition requires a concrete static implementing GlobalAlloc.

#[embassy_executor::main]
async fn main(_spawner: Spawner){
    // let stack = Stack::new();
    // stack.set(0, 234.5);
    // stack.set(1, 456.7);
    // stack.set(2, 987.6);
    // stack.get_x();
    // // for n in stack.data{
    //     // let handle = Rc::clone(&stack.data);
    // for i in 0..4 {
    //     defmt::println!("{}: {}", i, stack.data.borrow()[i]);
    // }    
    for i in 0..4 {
        defmt::println!("{}: {}", i, i);
    }  
}





// async fn main (_spawner: Spawner) {
//     stack::main_stack();
// }
    

    // stack.set(0, 234.5);
    // stack.set(1, 456.7);
    // stack.set(2, 987.6);
    // stack.get_x();
    // // for n in stack.data{
    //     // let handle = Rc::clone(&stack.data);
    // for i in 0..4 {
    //     info!("{}: {}", i, stack.get_x());//data.borrow()[i]);
    // }
 


// {
//     let stack = Stack::new();

//     let x = stack.x = 45.45;
//     info!("Stack[i]={}", x);

// }
    // stack::stack_test();

// }

// async fn old_main (_spawner: Spawner) {
//     let p = embassy_rp::init(Default::default());

//     // Give a quick flash on the RP2350 LED to show that the device is alive.
//     let pico_led = Output::new(p.PIN_25, Level::High);
//     let mut flash_led = FlashLed::new(pico_led, 20_000_000);
//     flash_led.flash();

//     // let mosi = p.PIN_19;
//     // let miso  = p.PIN_20;
//     // let display_cs = p.PIN_21;
//     // let clk = p.PIN_18;
//     // let reset  = p.PIN_28;
//     // let a0 = p.PIN_27;
//     let a0 = Output::new(p.PIN_27, Level::Low);   
//     let display_config = spi::Config::default();

//     let spi = Spi::new_blocking(p.SPI0, p.PIN_18, p.PIN_19, p.PIN_20, display_config.clone());
//     let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
//     let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(p.PIN_21, Level::High), display_config);
//     let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);
//     let mut page_buffer = GraphicsPageBuffer::new();
//     let display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, 
//                 SPI0, embassy_rp::spi::Blocking>, Output<'_>>, Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8> 
//                 = st7565::ST7565::new(display_interface, DOGL128_6)
//         .into_graphics_mode(&mut page_buffer);       
//     let reset_pin = Output::new(p.PIN_28, Level::Low);
//     // let font = MonoTextStyle::new(&FONT_9X18, BinaryColor::On);
//     let font = MonoTextStyle::new(&FONT_9X18, BinaryColor::On);
// // let font = MonoTextStyle::new(&PROFONT_14_POINT, BinaryColor::On);
    
//     let stacknames_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
//     let e_font = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
//     let number_style = DisplayStyle::E(4);
  
//     // let mut x = XLine::Number(1.0);
//     let mut x_str = String::<EDIT_LENGTH>::new();
//     x_str.push_str("ABC").unwrap();
    
//     // This struct holds the values used for viewing, not the ones calculated on.
//     // Let's see if that works.
//     let stack_view = StackView::new(
//         Some(x_str),
//         [0.113456, 2345.67, 89011., 123456.789],
//     );

//     // This struct holds the actual stack. Hmm, not sure about this.
//     // let mut stack = Stack::new();

//     let mut display = DisplayStruct::new(
//         display, //: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
//         reset_pin,  
//         font,// MonoTextStyle<'a, BinaryColor>,
//         stacknames_font, //: MonoTextStyle<'a, BinaryColor>,
//         e_font, //: MonoTextStyle<'a, BinaryColor>,
//         number_style,
//         stack_view
//     );

//     display.set_on(true);


//     let mut keyboard = Keyboard::new(
//         [
//             Input::new(p.PIN_2, Pull::Down),
//             Input::new(p.PIN_3, Pull::Down),
//             Input::new(p.PIN_4, Pull::Down),
//             Input::new(p.PIN_5, Pull::Down),
//             Input::new(p.PIN_6, Pull::Down),
//             Input::new(p.PIN_7, Pull::Down),
//             Input::new(p.PIN_8, Pull::Down),
//             Input::new(p.PIN_9, Pull::Down),
//         ],
//         [
//             Output::new(p.PIN_10, Level::Low),
//             Output::new(p.PIN_11, Level::Low),
//             Output::new(p.PIN_12, Level::Low),
//             Output::new(p.PIN_13, Level::Low),
//             Output::new(p.PIN_14, Level::Low),
//             Output::new(p.PIN_15, Level::Low),
//         ],
//     );

//     let mut state: State = State::Entry;
//     // let mut line_edit = LineEdit::new();
//     let mut calculate : Calculate = Calculate::new(); 

//     display.set_number_style(DisplayStyle::E(3));
    
//     // display.stack_view.set_format(DisplayStyle::E(3));
//     let mut line_edit = LineEdit::new();
//     // display.update_stack_display(None);
// // ******************************************************************************************** //
//     loop{
//         //100E6 is about once per second
//         delay(1_000_000); 
//         let key = keyboard.scan();
//         let key: Option<keyboard::KeyName> =  key.await;
//         if key.is_none(){
//             continue;
//         } else {
//             let key = key.unwrap();
//             info!("main: {} key pressed", key);         

//             match state {
//                 State::Entry => {
//                     info!("State: entry");
//                     if WORK_IN_ENTRY_MODE.contains(key) | ENTER_AND_EDIT_ENTRY_MODE.contains(key){

//                             info!("------Entry ");
//                             let entry_line = line_edit.process_number_keys(key);

//                             display.update_stack_display(entry_line);
//                     } else {
//                         let _x: f64 = line_edit.line.parse::<f64>().unwrap();
//  //                   stack.push(Some(x));
//                         state = State::Calculating;
//                         info!("Going to calculating in main");
//                         // calculate.process_calculate_key(key);
//                     }
//                 },
//                 State::Calculating => {
//                     info!("State: calculating - key is: {}", key);
//                     if ENTER_AND_EDIT_ENTRY_MODE.contains(key){
//                         state = State::Entry; info!("Change state to Entry.......");
//                         // display.update_stack_display(line_edit.process_number_keys(key));
//                         info!("In calculating, setting global: state to Entry for: {}", key);
//                         // This is also displayed on the LCD screen
//                     } else {
//                         info!("In calculating, process_key: {}", key);
//                         calculate.process_calculate_key(key);
//                     }
//                 },
//             }
//         }
//         // display.update_stack_display(None);
//     }
// }
