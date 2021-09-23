
fn greeting(greet: &str, name: &str){
    println!("{} {}, benvenidos!", greet, name);
}

fn add(x:i32, y:i32) -> i32 {
    x+y
}

pub fn run(){
    greeting("Piccolo", "Postino");
    println!("5+23={}", add(5,23));

    //Closure
    let add_nums = |n1: i8, n2: i8| n1 + n2;
    println!("Closure sum: 3+5={}", add_nums(3,5));

    let ten = 10;
    let add_ten = |n1: i32| n1 + ten;
    println!("Closure plus ten: {}", add_ten(7)); 
}