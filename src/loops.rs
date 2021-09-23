pub fn run(){
    let mut count = 0;

    // Infinite loop
    loop {
        count +=1;
        println!("Count:{}", count);

        if count > 20 {
            break;
        }
    }


    println!("While powered FizBuzz");
    count = 0;
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");

        }
        if count % 3 == 0 {
            println!("fizz");
        }
        if count % 5 == 0 {
            println!("buzz");
        }
        else{
            println!("{}", count);
        }

        count +=1;
    }

    println!("For powered FizBuzz");
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");

        }
        if x % 3 == 0 {
            println!("fizz");
        }
        if x % 5 == 0 {
            println!("buzz");
        }
        else{
            println!("{}", x);
        }
    }
}