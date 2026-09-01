#![no_std]
#![no_main]

use core::cell::RefCell;
//use core::sync::atomic::Atomic;
// use core::fmt::Write;

// use core::mem::MaybeUninit;

// use nostd::format;

use cortex_m::asm::delay;
// use defmt::*;
// use defmt::{Format};

use {defmt_rtt as _, panic_probe as _};

use defmt::info; //unnecessary?

mod display;
use display::DisplayStruct;
use display::DisplayStyle;
use display_interface_spi::SPIInterface;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
// use embassy_embedded_hal::shared_bus::SpiDeviceError;

use embassy_rp::gpio::{Input, Level, Output, Pull, AnyPin};
use embassy_rp::peripherals::{SPI0};
// use embassy_rp::{Peri, PeripheralType};
// use embassy_rp::rom_data;
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi}; //, ClkPin, Config, MisoPin, MosiPin


use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;
// use embassy_time::{Duration, Timer};

// text};

// use embedded_hal::spi::SpiDevice;
// use embedded_hal::digital::{InputPin, OutputPin};

use embassy_executor::Spawner;
// use embassy_rp::gpio;

use embedded_graphics::mono_font::ascii::{FONT_7X13, FONT_10X20};//, FONT_9X18, FONT_9X18_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{prelude::*};
use embedded_graphics::text::Text;

// use embedded_graphics::text::Text;
use enum_dispatch;

use heapless::String;
// use rp235x_hal as hal;

mod keyboard;
use keyboard::Keyboard;


// mod led;
// use led::FlashLedStruct;

mod line_edit;
use line_edit::{LineEdit};

mod setup;
// use setup::setup_display_hw;

use st7565::{GraphicsPageBuffer};
use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;

// use keyboard::EDIT_ENTRY;
mod stack;




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
enum State {
    // Undefined,
    Editing,
    Calculating,
}

trait OutputPin {}

trait OutputPins {
    type C0: OutputPin;
    type C1: OutputPin;
    type C2: OutputPin;
    type C3: OutputPin;
    type C4: OutputPin;
    type C5: OutputPin;
}

trait InputPin {}

trait InputPins {
    type R0: InputPin;
    type R1: InputPin;
    type R2: InputPin;
    type R3: InputPin;
    type R4: InputPin;
    type R5: InputPin;
    type R6: InputPin;
    type R7: InputPin;
}


// enum RowPins {
//     Row0(R0),
//     Row1(R1),
//     Row2(R2),
//     Row3(R3),
//     Row4(R4),
//     Row5(R5),
//     Row6(R6),
//     Row7(R7),
// }

struct FlashLedStruct {
    led: Output<'static>,
    delay: u32,
}

impl FlashLedStruct {
    fn new(led: Output<'static>, delay: u32) -> Self {
        Self { led, delay }
    }

    fn flash(&mut self) {
        self.led.set_high();
        delay(self.delay);
        self.led.set_low();
        delay(self.delay);
    }
}


#[embassy_executor::main]
async fn main (_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // let mut _buffer = String::<32>::new();

    info!("Started");

// On the RP235X, GPIO25 is connected to the user LED. (Datasheet page 9/24, checked 31/8/2026)
// https://pip-assets.raspberrypi.com/categories/1005-raspberry-pi-pico-2/documents/RP-008299-DS-3-pico-2-datasheet.pdf
    let pico_led = Output::new(p.PIN_25, Level::High);
    let mut flash_led = FlashLedStruct::new(pico_led, 20_000_000);
    flash_led.flash();

    
    let mosi = p.PIN_19;
    let miso  = p.PIN_20;
    let display_cs = p.PIN_21;
    let clk = p.PIN_18;
    let reset  = p.PIN_28;
    let a0_pin = p.PIN_27;
    let a0 = Output::new(a0_pin, Level::Low);   
    let display_config = spi::Config::default();
    let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
    let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);

       info!("display interface created");

    let mut page_buffer = GraphicsPageBuffer::new();
    let mut reset_pin = Output::new(reset, Level::Low);
    
    let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let stacknames_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);

    let mut stack = stack::Stack::new();
    let display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>, Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8> = st7565::ST7565::new(display_interface, DOGL128_6)
        .into_graphics_mode(&mut page_buffer);  
    let e_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);

    let number_style =  DisplayStyle::E(4);

    let mut display: DisplayStruct =  DisplayStruct::new(
        display,
        reset_pin,
        font,
        stacknames_font,
        e_font,
        number_style,
    );

// mut display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
//                 mut reset_pin: Output<'a>, 
//                 font: MonoTextStyle<'a, BinaryColor>,
//                 stack_names_font: MonoTextStyle<'a, BinaryColor>,
//                 e_font: MonoTextStyle<'a, BinaryColor>,
//                 // f_font: MonoTextStyle<'a, BinaryColor>,
//                 number_style: DisplayStyle,





    // display.reset(&mut reset_pin, &mut Delay).unwrap(); 

    // &mut display.display.reset_pin, &mut Delay).unwrap();

    // display.display.set_display_on(true);
    // let _ = display.display.flush();
    // display.display.set_display_on(true);
    // // display.update_stack_display(S);

let _ = Text::new("Hello world", Point::new(2, 2), font).draw(&mut display.display);
        



    // let mut number:String::<20> = String::new();
    // for c in "Some line".chars() {
    //     let _= number.push(c);
    // }
    // let num = num_to_string

    // // display.update_stack_display(Some(arb_line));

    // // self.
    // display.clear(BinaryColor::Off);





                      // This seems to work

    info!("About to display test code");

    // self.
    display.display.flush().unwrap();
let _ = Text::new("Hello world", Point::new(2, 2), font).draw(&mut display.display);


    loop{};//*********************************************************************************

    // Keyboard pins
    // let rows: [AnyPin; 8] = [*p.PIN_2.into() , *p.PIN_3.into(), *p.PIN_4.into(), *p.PIN_5.into(), *p.PIN_6.into(), *p.PIN_7.into(), *p.PIN_8.into(), *p.PIN_9.into()];
    // let cols: [AnyPin; 6] = [*p.PIN_10.into(), *p.PIN_11.into(), *p.PIN_12.into(), *p.PIN_13.into(), *p.PIN_14.into(), *p.PIN_15.into() ] ;
    

    let row1 = Input::new(p.PIN_2, Pull::Down);
    let row2 = Input::new(p.PIN_3, Pull::Down);
    let row3 = Input::new(p.PIN_4, Pull::Down);
    let row4 = Input::new(p.PIN_5, Pull::Down);
    let row5 = Input::new(p.PIN_6, Pull::Down);
    let row6 = Input::new(p.PIN_7, Pull::Down);
    let row7 = Input::new(p.PIN_8, Pull::Down);
    let row8 = Input::new(p.PIN_9, Pull::Down);

    let col1 = Output::new(p.PIN_10, Level::Low); 
    let col2 = Output::new(p.PIN_11, Level::Low);
    let col3 = Output::new(p.PIN_12, Level::Low);
    let col4 = Output::new(p.PIN_13, Level::Low);
    let col5 = Output::new(p.PIN_14, Level::Low);
    let col6 = Output::new(p.PIN_15, Level::Low);

    let rows = [row1, row2, row3, row4, row5, row6, row7, row8];
    let cols = [col1, col2, col3, col4, col5, col6];

    let mut keyboard = Keyboard::new(rows, cols);
    let mut line_edit = LineEdit::new();

// New ideaa (31/8/2026)
// Stack lives in main
// LineEdit only processes characters being entered. It returns:
// - a string that is the current entry value, or
// - an operation that can be undertaken on the stack
// or 



    loop{
        //100E6 is about once per second
        delay(10_000_000); 
        info!("before kbd scan");
        let key = keyboard.scan();
        let k: Option<keyboard::KeyName> =  key.await;
        if k.is_none(){
            continue;
        } else {
            
            let k = k.unwrap();
            info!("main: {} key pressed", k);         
            
            line_edit.process_key(k);      

            // Three outputs:
            // Option<String> containing current line entry string
            // 

            // This can enter number_entry, or enter operator entry, or stay
            // in the current mode. If we're entering number entry, return
            // the line as a string, and keep returning it until we leave
            // number entry. 

            // If key enters processing or line_edit is already in processing
            //
            // the value of the line





// Okay, now figure out how to carry on with the outputs from process_key()
            // if let Some(number) = result {
            //     info!("Some result in main: {}", &result.unwrap());
            //     display.push_stack(number);
            //     display.update_stack_display(None);
            // } else {
            //     info!("No result in main around line 200");
            // }

            // info!("Back in main loop");

            // number_edit
            // let number_str: String<20> = String::new();
            

            // display.update_stack_display(Some(number_str));
            // stack.swapxy();
            // stack.set_changed();                                            //
            //display.entry.editing = !display.entry.editing;
            // info!("Editing in main around line 226: {}\n\n", display.entry.editing);
                //100E6 is about once per second
        }
    }

}

// When hitting enter, the stack updates fine, but the status doesn't change to editing = false, 
//so the next time enter is hit, it goes into editing mode again.  Need to fix this. STOP EDITING MODE

// After entering a number and pushing it onto the stack, the bottom level
// of the stack should be in non-editing mode and showing the whole shebang.

// Add _ to maths editing line

