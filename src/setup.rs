// This file sets up all the hardware for the main program

use core::cell::RefCell;

use display_interface_spi::SPIInterface;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;

//use embassy_hal_internal::peripheral; // Not available publically?

use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_rp::gpio::{Level, Output, AnyPin, Pin,Input};
use embassy_rp::{Peri, Peripherals};
use embassy_rp::peripherals::SPI0;

use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi};

use st7565::ST7565;
use st7565::displays::DOGL128_6;
use st7565::GraphicsPageBuffer;
use st7565::modes::GraphicsMode;

use crate::display;


// pub fn setup_display_hw(p: &Peripherals)->display::ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>, Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8>{

//     let mosi = p.PIN_19;
//     let miso  = p.PIN_20;
//     let display_cs = p.PIN_21;
//     let clk = p.PIN_18;
//     let reset  = p.PIN_28;
//     let a0 = p.PIN_27;

//     let a0 = Output::new(a0, Level::Low);   

//     let display_config = spi::Config::default();
//     let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
//     let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
//     let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
//     let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);

//     let mut page_buffer = GraphicsPageBuffer::new();
//     let reset_pin = Output::new(reset, Level::Low);
//     // let stacknames_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);

//     let display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>, Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8> = st7565::ST7565::new(display_interface, DOGL128_6)
//         .into_graphics_mode(&mut page_buffer);  

//     display

// }

const NROWS: u8 = 9;
const NCOLS: u8 = 6;



// pub struct KeyboardMap<'a>{
//     rows: [Peri<'a, dyn Pin>; NROWS], 
//     cols: [Peri<'a,>; NCOLS]
// }








