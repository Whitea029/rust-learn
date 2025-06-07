fn main() {
    // 进制的字面量
    let a1: i32 = -125;
    let a2: i32 = 0xFF;
    let a3: i32 = 0o13;
    let a4: i32 = 0b10;
    println!("a1: {}, a2: {}, a3: {}, a4: {}", a1, a2, a3, a4);
    // max min 
    println!("i32::MAX: {}, i32::MIN: {}", i32::MAX, i32::MIN);
    println!("u32::MAX: {}, u32::MIN: {}", u32::MAX, u32::MIN);
    // float 
    let f1: f32 = 3.14;
    let f2: f64 = 3.14;
    println!("f1: {}, f2: {}", f1, f2);
    println!("Float are {:.1}, {:.1}", f1, f2);
    // bool
    let ok: bool = true;
    let fail: bool = false;
    println!("ok: {}, fail: {}", ok, fail);
    // char
    let c1: char = 'a';
    let c2: char = '中';
    let c3: char = '😊';
    println!("c1: {}, c2: {}, c3: {}", c1, c2, c3);
}
