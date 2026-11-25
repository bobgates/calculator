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
use crate::display::DisplayLine::T;
// use crate::keyboard::KeyName;
use crate::eng_fmt::FormatEng;
use crate::line_edit::{EDIT_LENGTH};

// use crate::stack::STACK;
use crate::stack::DISPLAY_STYLE;


use st7565::displays::DOGL128_6;
pub use st7565::ST7565;
use st7565::modes::GraphicsMode;

use crate::stack::Stack;
use crate::State::Calculating;
// use num_traits::float::FloatCore;

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
    E(usize),
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
impl DisplayStyle {
    fn to_letter(self, input: DisplayLine)->char{
        match input {
            DisplayLine::T => 'T',
            DisplayLine::Z => 'Z',
            DisplayLine::Y => 'Y',
            DisplayLine::X => 'X',
        }
    }

    fn fetch_all(&mut self) -> DisplayStyle{
        DisplayStyle::E(4)
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
    on: bool,
    // pub stack_view: &'a mut DisplayStackView,
}

impl <'a> DisplayStruct <'a>{
    pub fn new(mut display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
                mut reset_pin: Output<'a>, 
                font: MonoTextStyle<'a, BinaryColor>,
                stack_names_font: MonoTextStyle<'a, BinaryColor>,
                e_font: MonoTextStyle<'a, BinaryColor>,
                number_style: DisplayStyle,
                // stack_ref: &'a mut Stack,
                // stack_view: &'a mut DisplayStackView,
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
            on: true,
            // stack_view: stack_view,
        }
    }

   // Converts an f64 into a string with the correct number of significant figures, 
   // and returns the position of the 'E' if it is present    
    pub fn set_number_style(&mut self, ds: DisplayStyle){
        self.number_style = ds;
    }

    pub fn set_on(&mut self, on: bool) {
        self.on = on;

        let _ = self.display.flush();
        self.display.set_display_on(on).unwrap();


// This code can be in or out:
        // let num_str: String<EDIT_LENGTH> =  format!("{}", "Power on").unwrap();//Format!("{}".num);
        // let _ =Text::new(&num_str, Point::new(24, 18), self.font).draw(&mut self.display);
        // self.display.flush().unwrap(); 
    }

    pub fn is_on(&self)->bool{
        self.on
    }


    /// Takes an option to an entry line. If you want to show characters
    /// being added, send the growing string in the option. If you want to
    /// show the number in the x register, send a None.
    /// 
    /// Also uses two globals:
    /// `DISPLAY_STYLE` and `STACK` (which is accessed through `Stack.get_all()`)
    /// 
    /// # Arguments
    /// * `entry_line` - the input string wrapped in an Option
    /// 
    /// # Examples
    /// ```
    /// update_stack_display(None); // Writes the whole stack to the screen
    /// let entry_line = line_edit.process_number_keys(key).into();
    /// display.update_stack_display(line_edit.get_entry_line); //().into()); // This might be wrong!
    /// update_stack_display();
    /// ```
 
    pub fn update_stack_display(&mut self, entry_line: Option<String<EDIT_LENGTH>>) {
        
        self.display.clear(BinaryColor::Off);        

        let ds = DISPLAY_STYLE.lock().fetch_all();
        
        let st = Stack::get_all(); // This doesn't get numbers in current entry...

info!("usd----- x is {}, y is {}, z is {}, t is {}, changed is {}", st.0, st.1, st.2, st.3, st.4);


        let y_str = num_to_string(ds, &st.1);
        self.draw_one_line(Some(y_str),  DisplayLine::Y );            
        let z_str = num_to_string(ds, &st.2);
        self.draw_one_line(Some(z_str),DisplayLine::Z );            
        let t_str = num_to_string(ds, &st.3);
        self.draw_one_line(Some(t_str),DisplayLine::T );            

        let mut x_str: String<EDIT_LENGTH> = String::new();
        if entry_line.is_some(){ // We have a string already
            // info!("--- in update_stack_display: entry_line is some: ");
            x_str = entry_line.unwrap();
            
            for c in x_str.chars() {
                info!("x_str: {}",c);
            }
            let _ = x_str.push('_');
            // info!("--- leaving usd");
        } else {
            x_str = num_to_string(ds, &st.0);
        }
        self.draw_one_line(Some(x_str),  DisplayLine::X );    
        self.display.flush().unwrap();          
        
    }


        // info!("about to draw one line into x");
        // self.draw_one_line(Some(x_str), DisplayLine::X );
       // Flushes internal buffer to the display

    


    // returns string with letter replaced. Used to replace E with a space
    // and vice_versa
    pub fn replace_letter(&mut self, entry_line: &String<EDIT_LENGTH>, letter_out: char, letter_in: char) -> (String<EDIT_LENGTH>, Option<u8>) {

        let line = entry_line.clone();
        let mut e_pos:Option<u8>= None;
        let mut out:String<EDIT_LENGTH> = String::new();
        for (pos, c) in line.chars().enumerate(){
            if c == letter_out {
                if c =='E' {e_pos=Some(pos.try_into().unwrap())}
                let _ = out.push(letter_in);
            } else {
                let _ = out.push(c);
            }
        }
        (out, e_pos)
    }


//Implements notable traits: 
//Future<Output = <StringInner<usize, VecStorageInner<[MaybeUninit<u8>; _]>> as Future>::Output>, Iterator<Item = <StringInner<usize, VecStorageInner<[MaybeUninit<u8>; _]>> as Iterator>::Item>, Read, Write



    /// Draws a number on the stack, given a string representation of the line and a 
    /// targe display line. The target can take values from DisplayLine::X
    /// to DisplayLine::T.
    /// 
    /// # Arguments
    /// 
    /// * `entry_line` - the Option<String> you want to print. If it is none, this
    ///                  function does nothing, apart from log a whinge.
    /// * `target` - the line you'd like the number to appear on, DisplayLine::X to DisplayLine::T.
    /// 
    /// # Example
    /// 
    /// ```
    /// self.draw_one_line(Some(y_str),  DisplayLine::Y );   
    /// ```
    pub fn draw_one_line(&mut self, entry_line: Option<String<EDIT_LENGTH>>, target: DisplayLine){   // e_pos: Option<i32>,
  
        // info!("In draw_one_line");

        if entry_line.is_none(){
            info!("entry_line is none in dislay.draw_one_line");
            return;
        }

        let line = entry_line.unwrap();
        
        // info!("entry line is: {}", line);
        // for (i, c) in line.chars().enumerate(){
        //     info!("{}-{}", i,c);
        // }

        let (letter, label_bottom, number_bottom)  = match target {
            DisplayLine::X => {("x", LABEL_BOTTOM, NUMBER_BOTTOM)},
            DisplayLine::Y => {("y", LABEL_BOTTOM - LINE_SPACING, NUMBER_BOTTOM - LINE_SPACING)},
            DisplayLine::Z => {("z", LABEL_BOTTOM - 2*LINE_SPACING, NUMBER_BOTTOM - 2*LINE_SPACING)},
            DisplayLine::T => {("t", LABEL_BOTTOM - 3*LINE_SPACING, NUMBER_BOTTOM - 3*LINE_SPACING)},
        };

        let (line, e_pos) = self.replace_letter(&line , 'E', ' ');

        //info!(" epos in draw_one_line {}",e_pos);

        // let line = line.clone();
        let _= Text::new(letter, Point::new(NAME_LEFT, label_bottom), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, label_bottom), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&line, Point::new(NUM_LEFT, number_bottom), self.font).draw(&mut self.display);
        if e_pos.is_some() {
            let _ = Text::new("E", Point::new(NUM_LEFT + NUM_WIDTH * e_pos.unwrap()as i32 + 2, number_bottom-2), self.e_font).draw(&mut self.display);
        }
        let (_line, _e_pos)= self.replace_letter(&line, ' ', 'E');

    }

}

// Takes a number style, essentially Eng, Sci or Fixed and
// a number of significant digits and an f64, and returns a 
// string with the number in the specified format.


/// Converts an f64 into a neatly formatted number.
/// 
/// # Arguments
/// 
/// * `number_style` - A variable of type DisplayStyle that will ultimately format in:
///                    all - showing all decimals, 
///                    Sci(N) - scientific notation (1 digit to the left of the decimal point) and
///                             the exponent
///                    Eng(N) - the exponent is always a multiple of 3, which means the 
///                             mantissa can vary between 1 and 999
///                     
/// * `target` - the line you'd like the number to appear on, DisplayLine::X to DisplayLine::T.
/// DisplayLine::X is at the bottom (the HP has an upward growing stack)
/// 
/// # Example
/// 
/// ```
/// let result = num_to_string(Eng(3), 431235);
///  assert_eq!(result, 431.2E3);
/// 
/// 
/// 
/// ```
/// self.draw_one_line(Some(y_str),  DisplayLine::Y );   will place the number on the second row of the 
/// ```
pub fn num_to_string(number_style: DisplayStyle, number: &f64 )->String<EDIT_LENGTH>{//}, Option<i32>){
    if *number == 0.0 {
        let mut output: String<EDIT_LENGTH>=format!("").unwrap();
        let _ = output.push('0');
        let _ = output.push('.');
        // let mut pos=0;
        match number_style {
            DisplayStyle::E(sf) => {
                // pos = sf+2;
                for _ in 0..sf {
                    let _ = output.push('0');
                }
            },
            _ => { let _ = output.push('X');}       // Debug marker that other modes are not implemented
        }
        // let _ = output.push('_');
        // let _ = output.push('0');
        return output; //,Some(pos));
    } else {
        let a: String<EDIT_LENGTH>;
        match number_style {
            DisplayStyle::E(sf) => {
                a = number.format_eng(Some(sf));//.unwrap();

                let mut b: String<EDIT_LENGTH>=String::new();
;
                for (l, c) in a.chars().enumerate(){
                    if c == 'e' {
                        b.push(' ').unwrap();
                        // e_pos = Some(l.try_into().unwrap());
                    } else {
                        b.push(c).unwrap();
                    }
                }   
                return b       
            },
            DisplayStyle::_S(_sf) => {
                return format!("Not implemented").unwrap();
            },
            DisplayStyle::_FIXED => {
                return format!("Not implemented").unwrap();
            },
            DisplayStyle::_ALL => { 
                return format!("Not implemented").unwrap();
            }

        }
    }
}


// ZERO OUT the input string when enter is hit