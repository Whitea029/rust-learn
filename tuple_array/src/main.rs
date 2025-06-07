fn main() {
    // tuple 
    let tup: (i32, &str, f64) = (500, "hello", 6.4);
    println!("Tuple: {} {} {}", tup.0, tup.1, tup.2);
    
    let mut tup2: (i32, &str, f64) = (1000, "world", 3.14);
    tup2.0 = 2000; // 修改元组的第一个元素
    println!("Modified Tuple: {} {} {}", tup2.0, tup2.1, tup2.2);

    // ()
    let empty_tuple: () = ();
    println!("Empty Tuple: {:?}", empty_tuple);

    // array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?} {}", arr, arr.len());

    for elem in arr {
        println!("Array Element: {}", elem);
    }
    
    // ownership
    let arr2: [i32; 3] = [10, 20, 30];
    let arr3 = (2, "ff");
    println!("Array 2: {:?}, Array 3: {:?}", arr2, arr3);
    let arr2_ownership = arr2; // 所有权转移
    let arr3_ownership = arr3; // 所有权转移
    println!("Array 2 Ownership: {:?}, Array 3 Ownership: {:?}", arr2_ownership, arr3_ownership);

    // copy
    // move ownership
    let string_item = String::from("aa");
    let string_item_tt = string_item; // move ownership
    // println!("String Item: {}", string_item); // 这行会报错，因为所有权已转移
    println!("String Item TT: {}", string_item_tt);
}
