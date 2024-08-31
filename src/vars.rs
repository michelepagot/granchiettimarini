pub fn run(){
    let name = "Micio";
    let mut age = 39;
    println!("My name is {} and I'm {}", name, age);

    age = 40;
    println!("It is my birthday, I'm {}", age);

    const ID: i8 = 16;
    println!("ID:{}", ID);

    let (my_name, my_age) = ("Pino", 18);
    println!("My name is {} and I'm {}", my_name, my_age);
}
