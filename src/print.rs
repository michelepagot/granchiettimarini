pub fn run(){
    let x = 9;
    println!("Cia Ciao {} {}", x, "Gino");
    println!("Sopra la {0} la {1} campa, sotto la {0} la {1} crepa.", "panca", "capra");
    println!("My name is {name} and I'm {age} years old", name="Michele",  age=39);
    println!("Bin=b{:b} Hex=0x{:x} Octal={:0}", 20, 20, 20);
    println!("Debug {:?}", (1, (1.2, "no"), 1==2, 12 + 1));
}