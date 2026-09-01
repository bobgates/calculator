#![no_std]
#![no_main]

use core::{cell::RefCell, fmt::Display};
use core::mem::MaybeUninit;

use cortex_m::asm::delay;
// use defmt::*;
// use defmt::{Format};

use {defmt_rtt as _, panic_probe as _};

use defmt::info; //enables info! for debugging;

mod display;
use display::DisplayStruct;
use display::DisplayStyle;
use display_interface_spi::SPIInterface;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
// use embassy_embedded_hal::shared_bus::SpiDeviceError;

use embassy_rp::gpio::{Level, Output};
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::{SPI0};
// use embassy_rp::{Peri, PeripheralType};
use embassy_rp::rom_data;
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, ClkPin, Config, MisoPin, MosiPin, Spi};


use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
// use embassy_time::Delay;
// use embassy_time::{Duration, Timer};

//, text};

// use embedded_hal::spi::SpiDevice;
// use embedded_hal::digital::{InputPin, OutputPin};

use embassy_executor::Spawner;
// use embassy_rp::gpio;

use embedded_graphics::mono_font::ascii::{FONT_7X13, FONT_10X20, FONT_9X18, FONT_9X18_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::{Text, TextStyle};
use embedded_graphics::prelude::*;

use heapless::string::StringInner;
use heapless::{String, format};

mod keyboard;
use keyboard::Keyboard;
// use keyboard::keyboard;
// use rp235x_hal as hal;
mod line_edit;
// use line_edit::LineEdit;
use line_edit::LineEdit;
// 
use st7565::{GraphicsPageBuffer};
use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;

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
pub enum State {
    // Undefined,
    Editing,
    Calculating,
}
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

info!("Started");

    let pico_led = Output::new(p.PIN_25, Level::High);
    let mut flash_led = FlashLedStruct::new(pico_led, 20_000_000);
    flash_led.flash();

    let mosi = p.PIN_19;
    let miso  = p.PIN_20;
    let display_cs = p.PIN_21;
    let clk = p.PIN_18;
    let reset  = p.PIN_28;
    let a0 = p.PIN_27;

    let a0 = Output::new(a0, Level::Low);   
    let display_config = spi::Config::default();

    let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
    let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);



    let mut page_buffer = GraphicsPageBuffer::new();
    let reset_pin = Output::new(reset, Level::Low);
    let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let stacknames_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);

    let mut stack = stack::Stack::new();

    let display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>, Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8> = st7565::ST7565::new(display_interface, DOGL128_6)
        .into_graphics_mode(&mut page_buffer);   
info!("display hardware initialised");



    
    let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let stacknames_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
    let e_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
    let eline: Option<heapless::String<20>> = None;


    let number_style =  DisplayStyle::E(4);

    let mut display:DisplayStruct = DisplayStruct::new(
        display, //: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
        reset_pin, //: Output<'a>, 
        font,// MonoTextStyle<'a, BinaryColor>,
        stacknames_font, //: MonoTextStyle<'a, BinaryColor>,
        e_font, //: MonoTextStyle<'a, BinaryColor>,
        number_style,//: DisplayStyle,
    );
info!("display interface instantiated");

    display.set_on(true);

    // display.display.reset(&mut reset, &mut Delay).unwrap();
    let _ = display.display.flush();
    display.set_on(true);

    display.update_stack_display(None);

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

    let mut line_edit = LineEdit::new();


        // let num_str: String<20> =  format!("{}", num).unwrap();//Format!("{}".num);
        // let _ =Text::new(&num_str, Point::new(0, 13), font)
        //         .draw(&mut display);
     loop{
        //100E6 is about once per second
        delay(10_000_000); 
        let key = keyboard.scan();
        let k: Option<keyboard::KeyName> =  key.await;
        if k.is_none(){
            continue;
        } else {
            let k = k.unwrap();
            info!("main: {} key pressed", k);         
            // let (result, editing) =
             line_edit.process_key(k);      
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
            let number_str: String<20> = String::new();
            

            display.update_stack_display(Some(number_str));
            // stack.swapxy();
            // stack.set_changed();                                            //
            //display.entry.editing = !display.entry.editing;
            // info!("Editing in main around line 226: {}\n\n", display.entry.editing);
                //100E6 is about once per second
        }
    }







    loop{
        // info!("In loop");
        display.update_stack_display(None); 
        stack._swapxy();
        stack._changed();
        delay(100_000_000);
    }

}
