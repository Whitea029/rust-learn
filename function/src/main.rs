fn main() {
    let a = 5;
    let b = 10;
    let result = add(a, b);
    println!("The sum of {} and {} is {}", a, b, result);
    let x = 20;
    change_value(x);
    println!("The value of x is still {}", x); // x remains unchanged
    let mut y = 30;
    modify_value(&mut y);
    println!("After modify_value, y is now {}", y); // y is modified
    let p = Point { x: 1, y: 2 };
    print_point(p); 
    println!("Point p is at ({}, {})", p.x, p.y); // p remains unchanged
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn change_value(mut x: i32) {
    x += 10;
    println!("Inside change_value, x is now {}", x);
}

fn modify_value(x: &mut i32) {
    *x += 10;
    println!("Inside modify_value, x is now {}", x);
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn print_point(point: Point) {
    println!("Point coordinates: ({}, {})", point.x, point.y);
}