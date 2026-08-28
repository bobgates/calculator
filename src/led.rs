// Code to light the user LED onthe RP2350
use cortex_m::asm::delay;
use embassy_rp::gpio::Output;
use embassy_rp::Peri;
use embassy_rp::Peripherals;


pub struct FlashLedStruct {
    led: Output<'static>,
    delay: u32,
}


impl FlashLedStruct {
    pub fn new(p: Peripherals, delay: u32) -> Self {

        let led = Output::new(p.PIN_19, Level::High);

        Self { led, delay }
    }

    pub fn flash(&mut self) {
        self.led.set_high();
        delay(self.delay);
        self.led.set_low();
        delay(self.delay);
    }
}

 let mosi = p.PIN_19;