// This file sets up all the hardware for the main program

use core::cell::RefCell;

use display_interface_spi::SPIInterface;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_rp::Peri;
use embassy_rp::Peripherals;
use embassy_rp::peripherals::{PIN_25, SPI0};

use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi};

// let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
    // let display_interface: 
    // SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);



pub fn setup_display_hw(mut a0: Peri<'_>){
// ->SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'static, NoopRawMutex, embassy_rp::spi::Spi<'static, SPI0, embassy_rp::spi::Blocking>, Output<'static>>, Output<'static>>{
                                        
                                        
//                                         SPIInterface<SpiDeviceWithConfig<'static, NoopRawMutex, Spi<'static, SPI0, Blocking>, Output<'static>>, Output<'static>> {
// SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'static, NoopRawMutex, embassy_rp::spi::Spi<'static, SPI0, embassy_rp::spi::Blocking>, Output<'static>>, Output<'static>>
    let a0 = Output::new(a0, Level::Low);

    let mut mosi = Output::new(pin, Level::Low);
    // p.PIN_19;
    // let miso  = p.PIN_20;
    // let display_cs = p.PIN_21;
    // let clk = p.PIN_18;
    // let reset  = p.PIN_28;
    // let a0 = p.PIN_27;

    // let a0 = Output::new(a0, Level::Low);   

    // let display_config = spi::Config::default();
    // let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
    // let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    // let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
    // // return SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = 
    
    // SPIInterface::new(display_spi, a0);

}
