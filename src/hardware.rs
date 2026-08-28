// This file defines the board for the calculator.
// It includes creating the hardware and setting it up
// with the intent that if the hardware changes, most
// of the work can be done here.

use cortex_m::asm::delay;

use display_interface_spi::SPIInterface;

use embassy_embedded_hal::shared_bus::asynch::spi::SpiDeviceWithConfig;

use embassy_rp::gpio::{Input, Level, Output};
use embassy_rp::gpio;
use embassy_rp::Peripherals;
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi}; //, ClkPin, Config, MisoPin, MosiPin


use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;


use st7565::{GraphicsPageBuffer};
use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;

use crate::display::DisplayStruct;

use heapless::String;

struct Hardware<'a>{
    pub display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
    
}


// impl Board {
//     pub fn new()->Board{
//         self
// }
// }

// use embassy_rp::{
//     gpio::{self, Level},
//     peripherals::CORE1,
//     Peripherals,
// };

/// Represents the hardware components of the clock.


impl Default for Hardware<'_> {
    fn default() -> Self {
        let peripherals: Peripherals = embassy_rp::init(embassy_rp::config::Config::default());

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
    let reset  = p.PIN_28;
    let a0 = p.PIN_27;

    let a0 = Output::new(a0, Level::Low);   
    let display_config = spi::Config::default();

    let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
    let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);






    }

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






        // let led0 = gpio::Output::new(peripherals.PIN_2, Level::Low);
        // let led1 = gpio::Output::new(peripherals.PIN_3, Level::Low);
        // let button = gpio::Input::new(peripherals.PIN_13, gpio::Pull::Down);
        // let core1 = peripherals.CORE1;

        // Self {
        //     led0,
        //     led1,
        //     button,
        //     core1,
        // }