use std::vec;

fn main() {
    let bosed_point = Box::new(Point { x: 10, y: 20 });
    println!("Boxed Point: ({}, {})", bosed_point.x, bosed_point.y);

    let mut boxed_int = Box::new(5);
    println!("Boxed Integer: {}", boxed_int);
    *boxed_int += 10; // Dereference and modify the value
    println!("Modified Boxed Integer: {}", boxed_int);

    // move and clone 
    let x = vec![1, 2, 3];
    let y = x.clone(); // Cloning the vector
    println!("Original vector: {:?}", x);
    println!("Cloned vector: {:?}", y);

    let x = "whitea".to_string();
    let y = x.clone(); // Cloning the string
    println!("Original string: {}", x);
    println!("Cloned string: {}", y);

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // Move ownership of p1 to p2
    // println!("Point 1: ({}, {})", p1.x, p1.y); // This line would cause a compile error because p1 is moved
}

struct Point {
    x: i32,
    y: i32,
}