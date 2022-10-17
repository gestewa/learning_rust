fn main() {
    //Scalar Types
    
    /*
    
    Int
    
        Length	Signed	Unsigned
        -------------------------
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize

        Note: isize and usize types depend on the architecture of the computer
        your program is running on, which is denoted in the table as “arch”: 64
        bits if you’re on a 64-bit architecture and 32 bits if you’re on a 
        32-bit architecture.
        
        Note: The primary situation in which you’d use isize or usize is when 
        indexing some sort of collection.

        Number literal	Example
        -----------------------------
        Decimal	        98_222
        Hex	            0xff
        Octal	        0o77
        Binary	        0b1111_0000
        Byte (u8 only)   b'A'

        Handling overflow
            * Wrap in all modes with the wrapping_* methods, such as wrapping_add
            * Return the None value if there is overflow with the checked_* methods
            * Return the value and a boolean indicating whether there was overflow
                with the overflowing_* methods
            * Saturate at the value’s minimum or maximum values with saturating_*
                methods
    */
 
    /* 
    
    Float
    
        Length	Signed	Unsigned
        --------------------------
        32-bit	f32	    N/A
        64-bit	f64	    N/A
        
        Note: f64 is default b/c on modern CPUs it’s roughly the same speed as f32
        but is capable of more precision.
    
    */

    /* 
    
    Bool
    
        let t = true;
        let f: bool = false; // with explicit type annotation
    
    */ 
    
    /* 
    
    Char

        
        Length
        ---------
        32-bit
    
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';

        Note: char is a Unicode Scalar Value
        
        Note: Unicode Scalar Value ranges from U+0000 to U+D7FF and U+E000 to 
            U+10FFFF inclusive.
 
        Note: Unicode Scalar Value can be used to represent:
            * Accented letters
            * Chinese characters
            * Japanese characters
            * Korean characters
            * emoji
            * zero-width spaces 

    */

    // Compound Types

    /* 
    
    Tuple 
    
        Multiple types
        Fixed length
        Data allocated on the stack

        let x: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;

        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("The value of y is: {y}");
    */
    
    /* 
    
    Array 
    
        One type only
        Fixed length
        Data allocated on the stack

        let a = [1, 2, 3, 4, 5];
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let months = ["January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"];
        let a = [3; 5]; // a = [3, 3, 3, 3, 3];
        let first = a[0];
        let second = a[1];

        Note: Rust will panic if invalid index is accessed at runtime as it is
            focused on memory-safety

    */

}