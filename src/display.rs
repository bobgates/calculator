/* 
    The idea of this crate is that you give it the
    numbers you want on the stack, and it creates what
    needs to go on the display. For a number in entry
    mode, it just shows the current string. For all
    other numbers, which are already f64, it converts
    them to string according to the format rule of the
    day.

*/


use defmt::info;
// use embassy_rp::pio::program::InSource::X;
use core::f64;//, todo};
// use core::fmt::Write;
use display_interface_spi::SPIInterface;

use embassy_rp::gpio::Output;
use embassy_rp::peripherals::{SPI0};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;

use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{prelude::*};
use embedded_graphics::text::Text;

use heapless::{format, String}; 
use crate::line_edit::{EDIT_LENGTH};//LineEdit


use st7565::displays::DOGL128_6;
pub use st7565::ST7565;
use st7565::modes::GraphicsMode;

// use crate::stack::Stack;
use crate::State::Calculating;
// use crate::XLine::Number;
use num_traits::float::FloatCore;

const NUMBER_BOTTOM: i32 = 62;
const LABEL_BOTTOM: i32 = 61;
const LINE_SPACING: i32 = 15;
const NAME_LEFT: i32 = 1;
const COLON_LEFT: i32 = 6;
const NUM_LEFT: i32 = 15; 
const NUM_WIDTH: i32 = 9;       // 9 for 9x18 font
const X_LABEL_BOTTOM: i32 = LABEL_BOTTOM;
const Y_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - LINE_SPACING;
const Z_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 2*LINE_SPACING;
const T_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 3*LINE_SPACING;
const T_NUM_BOTTOM: i32 = NUMBER_BOTTOM - 3*LINE_SPACING;

// The HP42S has a 131x16 pixel display - two lines by 22 characters. They look something like 1.6x1, height to width
// The characters are 5 pixels wide and 7 high. with 1 pixel x spacing.
//
//The EA DOGL display I have is 128x64, which is slightly narrower and more than twice as high, but
// I probably won't use that height.

// It is possible to get 2.7" displays in 240x320...
#[derive(Copy, Clone, Debug)]
pub enum DisplayStyle{
    E(i32),
    _S(i32),
    _FIXED,
    _ALL,
}

pub enum DisplayLine{
    X,
    Y, 
    Z,
    T,
}
impl DisplayLine {
    fn to_letter(self, input: DisplayLine)->char{
        match input {
            DisplayLine::T => 'T',
            DisplayLine::Z => 'Z',
            DisplayLine::Y => 'Y',
            DisplayLine::X => 'X',
        }
    }
}

#[derive(Clone, Debug)]
pub struct DisplayStackView{
    pub x_str: Option<String<EDIT_LENGTH>>,
    pub xyzt: [f64;4],
    pub ds:DisplayStyle,
}

impl DisplayStackView{
    pub fn new( x: Option<String<EDIT_LENGTH>>, xyzt: [f64;4])->DisplayStackView {
        DisplayStackView{
            x_str: x,
            xyzt,
            ds: DisplayStyle::E(4),
        }
    }

    pub fn set_format(&mut self, ds: DisplayStyle) {
        self.ds = ds;
    }

    pub fn set_all(&mut self, a: Option<String<EDIT_LENGTH>>, xyzt: [f64; 4]) {
        self.x_str = a;
        self.xyzt = xyzt;
    }

    pub fn get_all(&self)->DisplayStackView{
        self.clone()
    }
}

// #[derive(EnumSetType, Debug, Format)]
// #[derive(Copy)]
pub struct DisplayStruct <'a>{
    pub display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
    reset_pin: Output<'a>,
    font: MonoTextStyle<'a, BinaryColor>,
    stack_names_font: MonoTextStyle<'a, BinaryColor>,
    e_font: MonoTextStyle<'a, BinaryColor>,
    number_style: DisplayStyle,
    eline : Option<String<EDIT_LENGTH>>,
    state: crate::State,
    pub stack_view: DisplayStackView,
}

impl <'a> DisplayStruct <'a>{
    pub fn new(mut display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
                mut reset_pin: Output<'a>, 
                font: MonoTextStyle<'a, BinaryColor>,
                stack_names_font: MonoTextStyle<'a, BinaryColor>,
                e_font: MonoTextStyle<'a, BinaryColor>,
                number_style: DisplayStyle,
                // stack_ref: &'a mut Stack,
                stack_view: DisplayStackView,
            ) -> Self {
        
        display.reset(&mut reset_pin, &mut Delay).unwrap();

info!("In display at startup");
info!("_____________________");

        display.flush().unwrap();       // Flushes internal buffer to the display

        Self { 
            display, 
            reset_pin,
            font,
            stack_names_font,
            e_font,
            number_style,
            eline: None,
            state: Calculating,
            stack_view: stack_view,
        }
    }

   // Converts an f64 into a string with the correct number of significant figures, 
   // and returns the position of the 'E' if it is present

    
    pub fn set_number_style(&mut self, ds: DisplayStyle){
        self.number_style = ds;
    }

    pub fn set_on(&mut self, on: bool) {

        let _ = self.display.flush();
        self.display.set_display_on(on).unwrap();

        let num_str: String<EDIT_LENGTH> =  format!("{}", "Screen on").unwrap();//Format!("{}".num);
        let _ =Text::new(&num_str, Point::new(0, 18), self.font).draw(&mut self.display);
         self.display.flush().unwrap(); 
    }



    // Updates the display with the current stack values and the current entry line
    // if it is active, or stack x value if it is not.
    pub fn update_stack_display(&mut self, entry_line: Option<String<EDIT_LENGTH>>) {
        
        self.stack_view.set_format(DisplayStyle::E(4));

        let sv = self.stack_view.get_all();
        let _y_str = num_to_string(sv.ds, &sv.xyzt[1]);
        let _z_str = num_to_string(sv.ds, &sv.xyzt[2]);
        let _t_str = num_to_string(sv.ds, &sv.xyzt[3]);

        // let mut x_str: String<EDIT_LENGTH> = String::new();
        // let mut epos: Option<i32> = None;
        if entry_line.is_some(){ // We have a string already
            let x_str:String<EDIT_LENGTH> = entry_line.unwrap();
            self.draw_one_line(Some(x_str), DisplayLine::X );
        } else {
            num_to_string(sv.ds, &sv.xyzt[0]);
        }
        self.display.flush().unwrap();       // Flushes internal buffer to the display

    }

    // Takes a option so that it'll work on empty strings.
    pub fn replace_letter(entry_line: &Option<String<EDIT_LENGTH>>, letter_out: char, letter_in: char)->Option<String<EDIT_LENGTH>>{
        let mut out:String<EDIT_LENGTH> = String::new();
        if entry_line.is_none(){
            return None;
        };
        let line = entry_line.clone().unwrap();
        for c in line.chars(){
            if c == letter_out {
                let _ = out.push(letter_in);
            } else {
                let _ = out.push(c);
            }
        }
        Some(out)
    }
  

    pub fn draw_one_line(&mut self, entry_line: Option<String<EDIT_LENGTH>>, target: DisplayLine){ 
  
        if entry_line.is_none(){
            info!("entry_line is none in dislay.draw_one_line");
            return;
        }

        let line = entry_line.clone().unwrap();
        
        info!("entry line is:");
        for (i, c) in line.chars().enumerate(){
            info!("{}-{}", i,c);
        }

        let (letter, label_bottom, number_bottom)  = match target {
            DisplayLine::X => {("x", LABEL_BOTTOM, NUMBER_BOTTOM)},
            DisplayLine::Y => {("y", LABEL_BOTTOM - LINE_SPACING, NUMBER_BOTTOM - LINE_SPACING)},
            DisplayLine::Z => {("z", LABEL_BOTTOM - 2*LINE_SPACING, NUMBER_BOTTOM - 2*LINE_SPACING)},
            DisplayLine::T => {("t", LABEL_BOTTOM - 3*LINE_SPACING, NUMBER_BOTTOM - 3*LINE_SPACING)},
        };

        // Replace the e in entry_line with a space
        let mut e_pos: Option<i32> = None;
        for (i, c) in line.chars().enumerate(){
            if c=='E' {
                e_pos = Some(i.try_into().unwrap());
            };
        }
        if e_pos.is_some(){
            info!("e_pos is {}", e_pos);
            Self::replace_letter(&Some(line.clone()) , 'E', ' ');
        }   

        // let line = line.clone();
        let _= Text::new(letter, Point::new(NAME_LEFT, label_bottom), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, label_bottom), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&line.clone(), Point::new(NUM_LEFT, number_bottom), self.font).draw(&mut self.display);
        if e_pos.is_some() {
            let _ = Text::new("E", Point::new(NUM_LEFT + NUM_WIDTH * e_pos.unwrap() + 3, number_bottom-2), self.e_font).draw(&mut self.display);
        }

        Self::replace_letter(&Some(line), ' ', 'E');
        
    }
}


// Takes a number style, essentially Eng, Sci or Fixed and
// a number of significant digits and an f64, and returns a 
// string with the number in the specified format.

pub fn num_to_string(number_style: DisplayStyle, number: &f64 )->(String<EDIT_LENGTH>, Option<i32>){
    if *number == 0.0 {
        let mut output: String<EDIT_LENGTH>=format!("").unwrap();
        let _ = output.push('0');
        let _ = output.push('.');
        let mut pos=0;
        match number_style {
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
        let mut a: String<EDIT_LENGTH>;
        match number_style {
            DisplayStyle::E(sf) => {
                let exponent: i32 = 1 + libm::log10(*number).floor() as i32;
                let mut before_dp = exponent % 3;  // This gives everything powers for 10^3, 10^-3, etc

                if before_dp ==0 {before_dp=3}; 
                if before_dp<0 {
                    before_dp=3+before_dp
                };
                let exp = exponent - before_dp;

                let n = (*number/(10.0_f64).powi(exponent-sf)).trunc()/10_f64.powi(sf-before_dp);
                // 1. the cutting off of the number to the correct number of significant figures
                // Leaves exp
                // if exp == 0

                // info!("--- {}E{}", n, exp);     // This produces a different string
                                                // to the format statement below,
                                                // info! gives .0 if there are no non-zero decimals
                                                // format just doesn't return no-zero decimals
                a = String::from(format!("{}E{}", n, exp).unwrap());
                // self.eline = Some(a.clone());

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

                let mut b: String<EDIT_LENGTH>=String::new();
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


// ZERO OUT the input string when enter is hit