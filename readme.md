 # simple_bf
 This program is a single file bf (brainfuck) interpreter written in rust.
 - Heap size of 30,000 bytes
 - Heap values wrap (mod 255)
 - Heap pointer wraps (mod 30,000)

 # Panics
 - when I/O operations fail
 - when arguments are invalid
 - when bf source code has any `[` or `]` that don't have a match

 # Usage
 Rust must be installed see: https://www.rust-lang.org/tools/install

 - <b>Default:</b> `simple_bf path/to/source.b`<br>
    program will halt no later than 1,000 execution cycles

 - <b>Specify halt time:</b> `simple_bf path/to/source.b 906`<br>
    program will halt after 906 execution cycles

 - <b>Examples:</b> `cd path/to/simple_bf`
   - `cargo run hello_world.b`
   - `cargo run echo.b`

# Overview
 bf consists of
 - list of values (heap)
 - value index
 - list of instructions (program)
 - instruction index

 There are 8 instructions, any other characters are ignored
 - `>` increment the value pointer
 - `<` decrement the value pointer
 - `+` increment the current value
 - `-` decrement the current value
 - `.` write the current value to the standard output stream
 - `,` store one byte from the standard input stream in the current value
 - `[` if the current value is 0 jump the instruction pointer after the corresponding `]`
 - `]` if the current value is not 0 jump the instruction pointer after the corresponding `[`
