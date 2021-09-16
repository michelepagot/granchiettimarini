pub fn run(){
    // Default is i32
    let i = 0;

    // Default is f64
    let f = 2.43;

    let isf: i64 = 1;
    
    println!("Max i32:{}", i32::MAX  );
    println!("Max i64:{}", i64::MAX  );

    let is_active = true;
    
    println!("-->__{:?}__", (i, f, isf, is_active));

    // Get boolean from an expressions
    let is_true = 5 == 5;
    println!("is_true:{}", is_true);

    let a = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (a, face));
}