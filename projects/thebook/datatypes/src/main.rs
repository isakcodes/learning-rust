fn main() {
    // i8, u8   signed, unsigned 8-bit
    // i16, u16
    // i32, u32
    // ...
    // i128, u128
    // isize, usize     arch (depends on computer architecture, 32 or 64 bit)

    // Rust requires us to specify type in this case, u32 type annotation
    let _guess: u32 = "42".parse().expect("Not a number!");

    // literals:
    // Decimal	        98_222
    // Hex  	        0xff
    // Octal	        0o77
    // Binary       	0b1111_0000
    // Byte (u8 only)	b'A'
    
    // When integers overflow, say u8 goes past 255, it will wrap (256=0, 257=1,...)
    // if compiling with --release flag. Else it panics with an error.
    
    // Floats
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // bools
    let _t = true;
    let _f: bool = false; // explicit type annotation

    // chars
    // - single quotes
    // - 4 byte size Unicode Scalar Value
    // - Can do emojis
    let _smile = '😀';

}
