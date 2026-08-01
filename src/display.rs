
// use cortex_m::asm::delay;
use defmt::info;
use core::f64;
// use core::num;
use display_interface_spi::SPIInterface;

use embassy_time::Delay;


// use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
// use embedded_graphics::mono_font::ascii::{FONT_7X13, FONT_10X20, FONT_9X18, FONT_9X18_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{prelude::*};
use embedded_graphics::text::Text;

use heapless::{format, String};
// use heapless::string::StringInner;

use crate::line_edit::LineEdit;

use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;


use embassy_rp::gpio::Output;
use embassy_rp::peripherals::{SPI0};

use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use crate::stack;
use num_traits::float::FloatCore;


const NAME_LEFT: i32 = 1;
const COLON_LEFT: i32 = 6;
const NUM_LEFT: i32 = 15; 
const NUM_WIDTH: i32 = 10;
const LINE_SPACING: i32 = 15;
const X_NUM_BOTTOM: i32 = 62;
const Y_NUM_BOTTOM: i32 = X_NUM_BOTTOM - LINE_SPACING;
const Z_NUM_BOTTOM: i32 = X_NUM_BOTTOM - 2*LINE_SPACING;
const T_NUM_BOTTOM: i32 = X_NUM_BOTTOM - 3*LINE_SPACING;
const X_LABEL_BOTTOM: i32 = 59;
const Y_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - LINE_SPACING;
const Z_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 2*LINE_SPACING;
const T_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 3*LINE_SPACING;

// The HP42S has a 131x16 pixel display - two lines by 22 characters. They look something like 1.6x1, height to width
// The characters are 5 pixels wide and 7 high.
//
//The EA DOGL display I have is 128x64, which is slightly narrower and more than twice as high, but
// I probably won't use that height.

// It is possible to get 2.7" displays in 240x320...

pub enum DisplayStyle{
    E(i32),
    _S(i32),
    _FIXED,
    _ALL,
}

// #[derive(EnumSetType, Debug, Format)]
// #[derive(Clone)]
pub struct DisplayStruct <'a>{
    pub display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
    reset_pin: Output<'a>,
    font: MonoTextStyle<'a, BinaryColor>,
    stack_names_font: MonoTextStyle<'a, BinaryColor>,
    e_font: MonoTextStyle<'a, BinaryColor>,
    // f_font: MonoTextStyle<'a, BinaryColor>,
    stack: stack::Stack,
    number_style: DisplayStyle,
    pub entry: LineEdit,
}

impl <'a> DisplayStruct <'a>{
    pub fn new(mut display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
                mut reset_pin: Output<'a>, 
                font: MonoTextStyle<'a, BinaryColor>,
                stack_names_font: MonoTextStyle<'a, BinaryColor>,
                e_font: MonoTextStyle<'a, BinaryColor>,
                // f_font: MonoTextStyle<'a, BinaryColor>,
                number_style: DisplayStyle,
            ) -> Self {
        
        display.reset(&mut reset_pin, &mut Delay).unwrap();

        Self { 
            display, 
            reset_pin,
            font,
            stack: stack::Stack::new(),
            stack_names_font,
            e_font,
            number_style,
            entry:  LineEdit::new(),
        }
    }

    pub fn num_to_string(&self, number: f64 )->(String<20>, Option<i32>){
        if number == 0.0 {
            let mut output: String<20>=format!("").unwrap();
            let _ = output.push('0');
            let _ = output.push('.');
            let mut pos=0;
            match self.number_style {
                DisplayStyle::E(sf) => {
                    pos = sf+2;
                    for _ in 0..sf {
                        let _ = output.push('0');
                    }
                },
                _ => { let _ = output.push('X');}       // Debug marker that other modes are not implemented
            }
            let _ = output.push('_');
            let _ = output.push('0');
            return (output,Some(pos));
        } else {
            let mut a: String<20>;
            match self.number_style {
                DisplayStyle::E(sf) => {

                    let exponent: i32 = 1 + libm::log10(number).floor() as i32;

                    let mut before_dp = exponent % 3;  // This gives everything powers for 10^3, 10^-3, etc

                    if before_dp ==0 {before_dp=3}; 
                    if before_dp<0 {
                        before_dp=3+before_dp
                    };
                    let exp = exponent - before_dp;

                    let n = (number/(10.0_f64).powi(exponent-sf)).trunc()/10_f64.powi(sf-before_dp);
                    // 1. the cutting off of the number to the correct number of significant figures
                    // Leaves exp
                    // if exp == 0

                    // info!("--- {}E{}", n, exp);     // This produces a different string
                                                    // to the format statement below,
                                                    // info! gives .0 if there are no non-zero decimals
                                                    // format just doesn't return no-zero decimals
                    a = String::from(format!("{}E{}", n, exp).unwrap());

                    // sf here is the number of significant figures to display, 
                    // but it is being interpreted as the number of decimal places 
                    // so we need to fix this the number accordingly

                    // There's an issue that if the number is an exact integer,
                    // it only has one place after the decimal: a zero. We need to 
                    // add more zeroes to show the full sig figs.
                    
                    // This comes down to subtracting the length of a from sf+1
                    // and adding that many zeroes
                    // Example: 
                    // 100.0
                    //.  that's character length 5, sf we want is 5,  so (sf+1)-len = 6-5 =
                    // add one zero.

                    let p = a.find("E").unwrap(); // must succeed, defined two lines above
                    // info!("Found E at {}",p);
                        
                    if !a.contains("."){
                        let required = sf+2 - a.len() as i32;
                        for _i in 0..required {
                            a.insert(p,'0').unwrap();
                        }
                        a.insert(p, '.').unwrap();
                    } 

                    let mut b: String<20>=String::new();
                    let mut e_pos: Option<i32> = None;
                    // info!("Contains E");
                    for (l, c) in a.chars().enumerate(){
                        if c == 'E' {
                            b.push(' ').unwrap();
                            e_pos = Some(l.try_into().unwrap());
                        } else {
                                b.push(c).unwrap();
                        }
                    }   
                    return(b, e_pos)       
                },
                DisplayStyle::_S(_sf) => {
                    return(format!("Not implemented").unwrap(), None)
                },
                DisplayStyle::_FIXED => {
                    return(format!("Not implemented").unwrap(), None)
                },
                DisplayStyle::_ALL => { 
                    return(format!("Not implemented").unwrap(), None)
                }

            }
        }
    }

    // Takes a string representing a number, returns
    // the position of the E if there is one and the input string
    // with the E replaced by a space

// change this so it changes the a in place


    pub fn string_to_string(&self, a: &String<20>)->(String<20>, Option<i32>){
        let p = a.find("E");
        if p == None {
            return (a.clone(), None);
        } else {
            let mut b: String<20>=String::new();
            let mut e_pos: Option<i32> = None;


            for (l, c) in a.chars().enumerate(){
                if c == 'E' {
                    b.push(' ').unwrap();
                    e_pos = Some(l.try_into().unwrap());
                } else {
                    b.push(c).unwrap();
                }
            }  
            return(b, e_pos)
        }
    }           

    pub fn set_on(&mut self, on: bool) {
        info!("switch display on");
        self.display.set_display_on(on).unwrap();

        let _ = self.display.flush();
        self.display.set_display_on(true).unwrap();

        let num_str: String<20> =  format!("{}", "Screen on").unwrap();//Format!("{}".num);
        let _ =Text::new(&num_str, Point::new(0, 13), self.font).draw(&mut self.display);
    }

    // Updates the display with the current stack values and the current entry line
    // if it is active, or stack x value if it is not.
    pub fn update_stack_display(&mut self, entry_line: Option<String<20>>) {
        self.display.clear(BinaryColor::Off);

        let (x, y, z, t) = self.stack.fetch_values();
        
        info!("x: {}, y: {}, z: {}, t: {}", x, y, z, t);
        info!("self.entry_line.chars");

        let mut outstr: String<20>=String::new();
        let mut e_pos: Option<i32> = None;
        let mut line : String<20> = String::new();


        let (x_buffer_str, epos) = 
        if entry_line.is_none(){
            self.num_to_string(x)
        } else {
            for (l, c) in entry_line.unwrap().chars().enumerate(){
                for c in line.chars(){ info!("s.e.l.c: {}.", c);}   
                if c == 'E' {
                    outstr.push(' ').unwrap();
                    e_pos = Some(l.try_into().unwrap());
                } else {
                    e_pos = None;
                    info!("c = {}",c);        
                    outstr.push(c).unwrap();  
                }  
            }
            (outstr.clone(), e_pos)
        }; 

        let (x_buffer_str, e_pos)=(outstr, e_pos);
        let _= Text::new("x", Point::new(NAME_LEFT, X_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, X_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&x_buffer_str, Point::new(NUM_LEFT, X_NUM_BOTTOM), self.font).draw(&mut self.display);
        if e_pos.is_some() {
            let _ = Text::new("E", Point::new(NUM_LEFT + NUM_WIDTH * e_pos.unwrap() + 2, X_NUM_BOTTOM-2), self.e_font).draw(&mut self.display);
        }

        let (y_buffer_str, e_pos) = self.num_to_string(y);
        let _= Text::new("y", Point::new(NAME_LEFT, Y_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Y_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&y_buffer_str, Point::new(NUM_LEFT, Y_NUM_BOTTOM), self.font).draw(&mut self.display);
        if e_pos.is_some() {
            let _ = Text::new("E", Point::new(NUM_LEFT + 10 * e_pos.unwrap() + 1, Y_NUM_BOTTOM-2), self.e_font).draw(&mut self.display);
        }

        let (z_buffer_str , e_pos)= self.num_to_string(z,);
        let _= Text::new("z", Point::new(NAME_LEFT, Z_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Z_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&z_buffer_str, Point::new(NUM_LEFT, Z_NUM_BOTTOM), self.font).draw(&mut self.display);
        if e_pos.is_some() {
            let _ = Text::new("E", Point::new(NUM_LEFT + 10 * e_pos.unwrap() + 2, Z_NUM_BOTTOM-2), self.e_font).draw(&mut self.display);
        }

        let (t_buffer_str, e_pos) = self.num_to_string(t);
        let _= Text::new("t", Point::new(NAME_LEFT, T_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, T_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&t_buffer_str, Point::new(NUM_LEFT, T_NUM_BOTTOM), self.font).draw(&mut self.display);
        if e_pos.is_some() {
        //     let _ = Text::new("E", Point::new(NUM_LEFT + 10 * e_pos.unwrap() + 2, T_NUM_BOTTOM-2), self.e_font).draw(&mut self.display);
        // }

        // self.display.reset(&mut self.reset_pin, &mut Delay).unwrap();
        self.display.flush().unwrap();       // Flushes internal buffer to the display

        }

    }
}