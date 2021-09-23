// Vectors are resizable Arrays 
use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1111111, 222222222, 33333333, 44444444, 555555555];
    let more_numbers: Vec<i64> = vec![1111111, 222222222, 33333333, 44444444, 555555555, 44444444, 555555555];

    // Change an element value
    numbers[3] = 0;

    println!("{:?}", numbers);

    // Get a single value
    println!("Single value 2: {}", numbers[2]);

    // Get length
    println!("Length:{}", numbers.len());

    // Vectors are are on the Heap
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
    println!("Larget vector occupies {} bytes", mem::size_of_val(&more_numbers));

    // Get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);
    let partial_slice: &[i32] = &numbers[0..2];
    println!("Partial slice: {:?}", partial_slice);

    //Add elements
    numbers.push(666666);
    numbers.push(777777);
    println!("Vector after a couple of push {:?}", numbers);

    numbers.pop();
    println!("Vector after a pop {:?}", numbers);

    // Loop through elements
    for x in numbers.iter() {
        println!("--> {}", x);
    }
    
    // Loop through elements and change them
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Muted numbers Vec: {:?}", numbers);

}