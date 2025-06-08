fn main() {
    // loop
    // loop {
    //     println!("This is an infinite loop!");
    //     // You can add a break condition here if needed
    // }

    // while
    let mut i = 0;
    while i < 10 {
        println!("Current value of i: {}", i);
        i += 1; // Increment i to avoid an infinite loop
    }

    // for 
    let arr = [1, 2, 3, 4, 5];
    for elem in arr {
        println!("Current element: {}", elem);
    }
    for i in 0..5 {
        println!("Current value of i in range: {}", i);
    }
    for i in 1..=5 {
        println!("Current value of i in inclusive range: {}", i);
    }

    // break
    for i in 0..10 {
        if i == 5 {
            println!("Breaking the loop at i = {}", i);
            break; // Exit the loop when i is 5
        }
        println!("Current value of i: {}", i);
    }

    'outer: loop {
        println!("outer");
        loop {
            println!("inner");
            break 'outer; // Breaks out of the outer loop
        }
    }

    // iterators
    let numbers = [1, 2, 3, 4, 5];
    let mut for_numbers = Vec::new();
    for &num in numbers.iter() {
        let item = num * num; // Square the number
        for_numbers.push(item); // Add the squared number to the vector
    }
    println!("Squared numbers: {:?}", for_numbers);
    // 迭代
    let numbers = [1, 2, 3, 4, 5];
    let iter_number: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared numbers using iterator: {:?}", iter_number);
}
