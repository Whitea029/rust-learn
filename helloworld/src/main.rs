fn main() {
    // 不可变与命名
    let _nice_count: i32 = 10; // 自动类型推断为 i32
    let _nice_number: i64 = 20;
    // 可变变量
    let mut _count: i32 = 30; // 可变变量
    _count += 1; // 修改可变变量的值
    // Shadowing
    let x: i32 = 5; // 初始值为 5
    {
        let x: i32 = x + 1; // Shadowing，x 的值被重新定义为 6
        println!("Shadowed x: {}", x); // 输出 6
    } // 外层的 x 仍然是 5
    println!("Outer x: {x}"); // 输出 5
}
