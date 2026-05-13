use core::fmt;
use lazy_static::lazy_static; 
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { //a lazily initialized static instance of the Writer struct that can be used throughout the kernel to write to the VGA buffer
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black), //set the default color code to yellow text on a black background
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, //the VGA buffer is located at memory address 0xb8000, so we create a mutable reference to it using an unsafe block
    });
}


#[allow(dead_code)] //disables warnings for unused code, which is common in low-level programming
#[derive(Debug, Clone, Copy, PartialEq, Eq)] //derives traits for the Color enum, allowing it to be printed, copied, and compared
#[repr(u8)]
pub enum Color { //specifies the color codes for the VGA text mode
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] 
struct ColorCode(u8); //a wrapper around a u8 that represents a color code for the VGA text mode

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode { //creates a new ColorCode from a foreground and background color
        ColorCode((background as u8) << 4 | (foreground as u8)) //the background color is stored in the high 4 bits and the foreground color in the low 4 bits
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] 
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] //ensures that the ColorCode struct has the same memory layout as a u8, which is important for writing to the VGA buffer
struct Buffer { //represents the VGA text mode buffer, which is a 2D array of ScreenChar structs
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], //the characters in the buffer, stored as a 2D array of ScreenChar structs
}


pub struct Writer { //a struct that can write characters to the VGA text mode buffer
    column_position: usize, //the current column position of the writer
    color_code: ColorCode, //the color code used for writing characters
    buffer: &'static mut Buffer, //a mutable reference to the VGA text mode buffer
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) { //writes a single byte to the VGA buffer
        match byte {
            b'\n' => self.new_line(), //if the byte is a newline character, move to a new line
            byte => {
                if self.column_position >= BUFFER_WIDTH { //if we've reached the end of the line, move to a new line
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1; //the row to write to is always the last row of the buffer
                let col = self.column_position; //the column to write to is determined by the current column position
                self.buffer.chars[row][col].write(ScreenChar { //write the character and color code to the buffer
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1; //move to the next column
            }
        }
    }

    pub fn write_string(&mut self, s: &str) { //writes a string to the VGA buffer by writing each byte of the string
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), //if the byte is a printable ASCII character or a newline, write it to the buffer
                _ => self.write_byte(0xfe), //if it's not a printable ASCII character, write a placeholder character (0xfe) to the buffer
            }
        }
    }

    fn new_line(&mut self) { //moves all lines up by one and clears the last line
        for row in 1..BUFFER_HEIGHT { //for each row starting from the second row
            for col in 0..BUFFER_WIDTH { //for each column
                let character = self.buffer.chars[row][col].read(); //read the character from the current row and column
                self.buffer.chars[row - 1][col].write(character); //write it to the previous row, effectively moving it up
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1); //clear the last row after moving everything up
        self.column_position = 0; //reset the column position to the beginning of the line
    }
    fn clear_row(&mut self, row:usize){ //clears a row by writing spaces with the current color code to each column in the specified row
        let blank = ScreenChar {
            ascii_character: b' ', //a space character
            color_code: self.color_code, //the current color code
        };
        for col in 0..BUFFER_WIDTH { //for each column in the row
            self.buffer.chars[row][col].write(blank); //write the blank character to clear it
        }
    }
}


impl fmt::Write for Writer { //implement the fmt::Write trait for our Writer struct, allowing us to use the write! macro to write formatted strings to the VGA buffer
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s); //write the string to the VGA buffer using our write_string method
        Ok(())
    }
}

#[macro_export] 
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*))); //use the format_args! macro to format the arguments and pass them to the
    //internal _print function, which will write the formatted string to the VGA buffer
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n")); 
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*))); //use the format_args! macro to format the arguments and pass them to the print macro, adding a newline at the end
}

#[doc(hidden)] 
pub fn _print(args: fmt::Arguments) { 
    use core::fmt::Write; 
    WRITER.lock().write_fmt(args).unwrap(); 
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}