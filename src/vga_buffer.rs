use volatile::Volatile;

// restrict the compiler to throw a warning for unused variant and dervied Copy, clone, Debug, 
// PartialEq & Eq traits, which will enable copy semantics for the type.
#[allow(dead_code)] 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // We’ve used enum to define colors and because of repr(u8) attribute, each enum variant will store as an u8
pub enum Color {
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

// structure contains foreground as well as background color and we used repr(transparent) attribute to ensure it has the same data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// structure that represents screen character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color_code: ColorCode,
}

// VGA text buffer is a two-dimensional array with 25 rows and 80 columns and to represent this we provided BUFFER_HEIGHT & BUFFER_WIDTH.
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// structure is for text buffer
#[repr(transparent)]
struct Buffer {
    // chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Writer structure which has column_position, color_code, & buffer
// the first thing we need is the data, the next thing will be the color 
// of the data and the last will be the column position like where we need to 
// write it we don’t need the row position because the writer will always write 
// to the last line.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    // implementation of the writer structure by adding one method for now which is write_byte. 
    // This method pertains to writing byte into the screen.
    /// Writes an ASCII byte to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
   
                // we are decreasing buffer height by 1 because the writer will always write to the last line
                let row = BUFFER_HEIGHT - 1;

                // then we are fetching the column position, text color
                let col = self.column_position;
                let color_code = self.color_code;

                // after that we are assigning byte and color to the buffer.chars to print the data
                self.buffer.chars[row][col].write(ScreenChar {
                    character: byte,
                    color_code,
                });

                // at last, we are incrementing column position to shift the pointer.
                self.column_position += 1;
            }
        }
    }

    /// Writes the given ASCII string to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character. Does **not**
    /// support strings with non-ASCII characters, since they can't be printed in the VGA text
    /// mode.
    pub fn write_string(&mut self, string_data: &str) {
        // we’ve converted string into bytes
        for byte in string_data.bytes() {
            // match expression, we differentiate the ASCII bytes because Rust strings are UTF-8 by default, 
            // as provided string might contain bytes that are not supported by the VGA test buffer.
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
                // for the unsupported bytes, we just provided 0xfe hex code
            }
        }
    }

    /// Shifts all lines one line up and clears the last row.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a row by overwriting it with blank characters.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// Testing function
// This function first creates a new Writer that points to the VGA buffer at 0xb8000
pub fn print_data() {
    // Then we cast the integer 0xb8000 as a mutable raw pointer.
    // Then we convert it to a mutable reference by dereferencing it (through *) and immediately borrowing it again (through &mut).
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        // This conversion requires an unsafe block since the compiler can’t guarantee that the raw pointer is valid.
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    // And at last, we called the write_byte() method by providing a byte.
    // THE POLYGLOT PROGRAMMER OS
    writer.write_byte(b'T');
    writer.write_byte(b'H');
    writer.write_byte(b'E');
    writer.write_byte(b' ');
    writer.write_byte(b'P');
    writer.write_byte(b'O');
    writer.write_byte(b'L');
    writer.write_byte(b'Y');
    writer.write_byte(b'G');
    writer.write_byte(b'L');
    writer.write_byte(b'O');
    writer.write_byte(b'T');
    writer.write_byte(b' ');
    writer.write_byte(b'P');
    writer.write_byte(b'R');
    writer.write_byte(b'O');
    writer.write_byte(b'G');
    writer.write_byte(b'R');
    writer.write_byte(b'A');
    writer.write_byte(b'M');
    writer.write_byte(b'M');
    writer.write_byte(b'E');
    writer.write_byte(b'R');
    writer.write_byte(b' ');
    writer.write_byte(b'O');
    writer.write_byte(b'S');
}