pub fn run() {
    // Different type data aggregation: Max 12 elements ??
    let person: (&str, &str, i8) = ("Pino", "Bologna", 0);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let voglidipiu: (i8,i8,i8,i8,i8,i8,i8,i8,i8,i8,i8,i8,i8) = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    println!("Voglio di piu' has {} elements", voglidipiu.12);
}