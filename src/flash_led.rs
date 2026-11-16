// use crate Output;
use cortex_m::asm::delay;
use embassy_rp::gpio::Output;

pub struct FlashLed<'a>{
    led: &'a mut Output<'static>,
    delay: u32,
}

impl <'a> FlashLed <'a> {
    pub fn new(led: &'a mut Output<'static>, delay: u32) -> Self {
        Self { led, delay }
    }

    pub fn flash(&mut self) {
        self.led.set_high();
        delay(self.delay);
        self.led.set_low();
        delay(self.delay);
    }
}
