// use crate Output;
use cortex_m::asm::delay;
use embassy_rp::gpio::Output;

pub struct FlashLed {
    led: Output<'static>,
    delay: u32,
}

impl FlashLed {
    pub fn new(led: Output<'static>, delay: u32) -> Self {
        Self { led, delay }
    }

    pub fn flash(&mut self) {
        self.led.set_high();
        delay(self.delay);
        self.led.set_low();
        delay(self.delay);
    }
}
