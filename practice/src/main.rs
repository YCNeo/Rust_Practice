fn main() {
    let mut tup = (50, 1.0, "hello", true);
    println!("Tuple: {:?}", tup);
    println!("Tuple: {:#?}", tup);
    println!("Tuple: {tup:#?}");
    tup.0 = 100; // Change the first element
    println!("Updated Tuple: {:?}", tup);
    let (x, y, z, i) = tup; // Ignore the last element
    println!("x: {}, y: {}, z: {}, i: {i}", x, y, z);
    
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    println!("Array: {:#?}", arr);
    println!("First element: {}", arr[0]);
    println!("Last element: {}", arr[arr.len() - 1]);
    println!("Array length: {}", arr.len());
    println!("Array is empty: {}", arr.is_empty());
    println!("Array contains 3: {}", arr.contains(&3));
}
