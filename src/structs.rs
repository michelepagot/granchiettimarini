// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct TColor(u8,u8,u8);

pub fn run(){
    let mut c = Color { red: 255, green: 0, blue:100};
    println!("Color: {} {} {}", c.blue, c.green, c.red);

    c.red = 0;
    println!("Color: {} {} {}", c.blue, c.green, c.red); 

    let mut tc = TColor(100,200,50);
    println!("TColor: {} {} {}", tc.0, tc.1, tc.2); 

    tc.2 = 99;
    println!("TColor: {} {} {}", tc.0, tc.1, tc.2);
}