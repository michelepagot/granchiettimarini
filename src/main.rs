mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditional;
mod loops;
mod functions;
mod pointers;
mod structs;


fn main() {
    println!("--------------- PRINT ---------------");
    print::run();
    println!("------------- PRINT DONE -------------");

    println!("--------------- VARS ---------------");
    vars::run();
    println!("------------- VARS DONE -------------");

    println!("--------------- TYPES ---------------");
    types::run();
    println!("------------- TYPES DONE -------------");

    println!("--------------- STRINGS ---------------");
    strings::run();
    println!("------------- STRINGS DONE -------------");

    println!("--------------- TUPLES ---------------");
    tuples::run();
    println!("------------- TUPLES DONE -------------");

    println!("--------------- ARRAYS ---------------");
    arrays::run();
    println!("------------- ARRAYS DONE -------------");

    println!("--------------- VECTORS ---------------");
    vectors::run();
    println!("------------- VECTORS DONE -------------");

    println!("--------------- CONDITIONAL ---------------");
    conditional::run();
    println!("------------- CONDITIONAL DONE -------------");

    println!("--------------- LOOPS ---------------");
    loops::run();
    println!("------------- LOOPS DONE -------------");

    println!("--------------- FUNCTIONS ---------------");
    functions::run();
    println!("------------- FUNCTIONS DONE -------------");

    println!("--------------- POINTERS ---------------");
    pointers::run();
    println!("------------- POINTERS DONE -------------");

    println!("--------------- STRUCTS ---------------");
    structs::run();
    println!("------------- STRUCTS DONE -------------");
}
