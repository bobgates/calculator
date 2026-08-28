// This file defines the board for the calculator.
// It includes creating the hardware and setting it up
// with the intent that if the hardware changes, most
// of the work can be done here.

use core::cell::RefCell;
use cortex_m::asm::delay;
use defmt::info;
use display_interface_spi::SPIInterface;
use crate::display::DisplayStruct;

use embassy_embedded_hal::shared_bus::asynch::spi::SpiDeviceWithConfig;

use embassy_rp::gpio::{Input, Level, Output};
use embassy_rp::gpio;
use embassy_rp::Peripherals;
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::Blocking;
use embassy_rp::spi;
use embassy_rp::spi::{Async, Spi}; //, ClkPin, Config, MisoPin, MosiPin

// use embassy_sync::blocking_mutex::raw::RawMutex;
// use embassy_sync::blocking_mutex::raw;

// use embassy_sync::blocking_mutex::Mutex;
// use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;

use heapless::String;


type SpiBus = Mutex<NoopRawMutex, Spi<'static, spi::Async>>;

use static_cell::StaticCell;


use st7565::{GraphicsPageBuffer};
use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;








// impl <'a> DisplayStruct <'a>{
//     pub fn new(mut display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
//                 mut reset_pin: Output<'a>, 
//                 font: MonoTextStyle<'a, BinaryColor>,
//                 stack_names_font: MonoTextStyle<'a, BinaryColor>,
//                 e_font: MonoTextStyle<'a, BinaryColor>,
//                 // f_font: MonoTextStyle<'a, BinaryColor>,
//                 number_style: DisplayStyle,
//                 stack: &'a mut Stack,
//             ) -> Self {
        
//         display.reset(&mut reset_pin, &mut Delay).unwrap();

//         Self { 
//             display, 
//             // reset_pin,
//             font,
//             stack: stack, //Stack::new(),    // No - you were just handed a stack!
//             stack_names_font,
//             e_font,
//             number_style,
//             eline: None,
//             // _entry: & LineEdit::new(stack),
//         }
//     }  
// }

/// Represents the hardware components of the clock.
pub struct Hardware<'a> {
    display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,        
}


impl Hardware<'_> {
    pub fn new<'b>() -> Self {
        // let peripherals: Peripherals = embassy_rp::init(embassy_rp::config::Config::default());

        let p = embassy_rp::init(Default::default());
        let mut _buffer = String::<32>::new();



        info!("Started");

        let pico_led = Output::new(p.PIN_25, Level::High);
        let mut flash_led = FlashLedStruct::new(pico_led, 20_000_000);
        flash_led.flash();

    let mosi = p.PIN_19;
    let miso  = p.PIN_20;
    let display_cs = p.PIN_21;
    let clk = p.PIN_18;
    let a0 = p.PIN_27;

    let a0 = Output::new(a0, Level::Low);   
    let spi_cfg = spi::Config::default();

    let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, spi_cfg);
    
    let spi_bus: Mutex<NoopRawMutex, RefCell<Spi<'_, SPI0, Blocking>>> = Mutex::new(RefCell::new(spi));

    
    // let spi_bus: Mutex<NoopRawMutex, _>      = Mutex::new(RefCell::new(Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone())));

let spi_bus: Mutex<NoopRawMutex, RefCell<Spi<'_, SPI0, Blocking>>> = Mutex::new(RefCell::new(Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone())));

    // let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
    let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);









        // let mosi = p.PIN_19;
        // let miso  = p.PIN_20;
 
        // let clk = p.PIN_18;
        
        // let a0 = p.PIN_27;
        // let a0 = Output::new(a0, Level::Low);   
        // let display_config = spi::Config::default();

        // let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
        // let spi_bus= Mutex::new(RefCell::new(spi));
        // //let spi_bus: Mutex<raw, _> = Mutex::new(RefCell::new(spi));
        // // let spi_bus: Mutex<NoopRawMutex, Spi<'_, SPI0, Blocking>> = Mutex::new(spi);
       
        // let display_spi=
        //         SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
        // let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Async>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);

        // let mut page_buffer = GraphicsPageBuffer::new();
        // let reset  = p.PIN_28;
        // let reset_pin = Output::new(reset, Level::Low);
        // // let stacknames_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);

        // let display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::asynch::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>, Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8> = st7565::ST7565::new(display_interface, DOGL128_6)
        //             .into_graphics_mode(&mut page_buffer);

        Self{
            display
        }
    }

let display: ST7565<SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8>
calculator::hardware::Hardware
display:     ST7565<SPIInterface<SpiDeviceWithConfig<'a, NoopRawMutex, Spi<'a, SPI0, Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>





//              embassy_embedded_hal::shared_bus::asynch::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Async>, Output<'_>>: embedded_hal::spi::SpiDevice`
// SPIInterface<embassy_embedded_hal::shared_bus::asynch::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Async>, Output<'_>>, Output<'_>>: display_interface::WriteOnlyDataCommand`

//    embassy_embedded_hal::shared_bus::asynch::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>: embedded_hal::spi::SpiDevice
    

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


