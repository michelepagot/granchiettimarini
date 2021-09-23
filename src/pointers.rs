pub fn run(){
    //With Primitive types like array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: arr1={:?} arr2={:?}", arr1, arr2);

   //With Non-Primitive types  like vector
   let vec1  = vec![1,2,3];
   let vec2 = &vec1;  // notice the need of using &

   println!("Values: vec1={:?} vec2={:?}", vec1, vec2);
}