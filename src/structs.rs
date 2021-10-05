// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct TColor(u8,u8,u8);

struct Person{
  first_name: String,
  last_name: String
}

impl Person{
  fn create(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    } 
  }

  fn full_name(&self) -> String {
    format!("Mr. {} {}", self.first_name, self.last_name)
  }
}


pub fn run(){
    let mut c = Color { red: 255, green: 0, blue:100};
    println!("Color: {} {} {}", c.blue, c.green, c.red);

    c.red = 0;
    println!("Color: {} {} {}", c.blue, c.green, c.red); 

    let mut tc = TColor(100,200,50);
    println!("TColor: {} {} {}", tc.0, tc.1, tc.2); 

    tc.2 = 99;
    println!("TColor: {} {} {}", tc.0, tc.1, tc.2);

    let p = Person::create("Peppino", "Paperino");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Format Person {}", p.full_name());

}
