// There are many ways to define modules and factor them into files. 
// https://stackoverflow.com/questions/69140355/is-it-possible-to-avoid-using-mod-rs-files

// this is the same as a mod.rs file in the front_of_house directory, or a 
//      front_of_house.rs file like the middle_of_house.rs file in the src/
//      directory. This is fine for small projects, as is the m_o_h.rs in src/
mod front_of_house {
    pub mod hosting;
}

mod back_of_house;
mod outside_of_house;
mod middle_of_house;

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    back_of_house::ordering::deliver_order();
    back_of_house::ordering::fix_incorrect_order();
    middle_of_house::cat::catt();
    outside_of_house::dog::dogss();
}
fn main() {

    /* New keywords

        * use
        * pub
        * mod
        * self
        * super
    
    */
    /* Definitions
    
    Crate
        - the smallest unit of code that the rust compiler will consider
        * Crates can contain modules (mod), and the modules may be defined in 
            other files that get compiled with the crate
        * Can be a a binary crate or a library crate

    Crate root
        - a rust source file that the compiler starts from and makes the root 
            module of your crate
        * lib.rs or main.rs
        
    Package
        - a bundle of 1+ crates that provide a set of functionality
        * contains a Cargo.toml file that describes how to build those
        crates
        * A package can contain as many binary crates as you like, but at
        most only one library crate. A package must contain at least
        one crate, whether that’s a library or binary crate.
    
    See the Rust API Guidelines on best practices for writing libraries & 
        making public APIs https://rust-lang.github.io/api-guidelines/

    Path
        * directions to code
        * e.g. an Asparagus type in the garden vegetables module
            crate::garden::vegetables::Asparagus.
        * Once a module is part of your crate, you can refer to code in that
            module from anywhere else in that same crate, as long as the 
            privacy rules allow, using the path to the code. 

    Private vs public
        * Code within a module is private from its parent modules by default.
        * To make a module public, declare it with pub mod instead of mod.
        * To make items within a public module public as well, use pub before
            their declarations.
        * struct's properties are private by default. 
        * enum's variants are public

    `use`
        - Within a scope, the use keyword creates shortcuts to items
        * In any scope that can refer to crate::garden::vegetables::Asparagus,
            you can create a shortcut with 
            use crate::garden::vegetables::Asparagus; 
            and from then on you only need to write Asparagus to make use of 
            that type in the scope.

    */

    /* Importing & exporting

    idiomatic when importing a function to import the function’s parent module into scope with use means we have to specify the parent module when ca lling the function
    b/c it makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.

        
        e.g. 
            use crate::front_of_house::hosting;

            mod customer {
                pub fn eat_at_restaurant() {
                    hosting::add_to_waitlist();
                }
            }

    idiomatic when importing in structs, enums, and other items to import the full path
        
        e.g.
            use std::collections::HashMap;

            fn main() {
                let mut map = HashMap::new();
            }

        note: we can also alias an import locally

        e.g.
            use std::io::Result as IoResult;

    We can group imports from the same package

        e.g. 
            use std::cmp::Ordering;
            use std::io;
            // same as above
            use std::{cmp::Ordering, io};
        e.g.
            use std::io;
            use std::io::Write;
            // same as above
        use std::io::{self, Write};

    We can import with glob ( brings in all public exports )

        e.g.
            use std::collections::*;

    We can re-export. this can be helpful when the internal structure of your
        code is different from how programmers calling your code would think
        about the domain.

        e.g.
            mod front_of_house {
                pub mod hosting {
                    pub fn add_to_waitlist() {}
                }
            }

            pub use crate::front_of_house::hosting;

    */
}