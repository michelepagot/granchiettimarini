pub fn run(){
    // Fixed in size and type 
    let mut numbers: [i64; 5] = [1111111, 222222222, 33333333, 44444444, 555555555];

    // Change an element value
    numbers[3] = 0;

    println!("{:?}", numbers);

    // Get a single value
    println!("Single value 2: {}", numbers[2]);

    // Get length
    println!("Length:{}", numbers.len());

    // Array are on the stack
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i64] = &numbers;
    println!("Slice: {:?}", slice);
    let partial_slice: &[i64] = &numbers[0..2];
    println!("Partial slice: {:?}", partial_slice);

}