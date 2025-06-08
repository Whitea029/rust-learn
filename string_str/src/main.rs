fn main() {
    // String &str
    let name = String::from("Value Cpp");
    // String::from()
    // to_string()
    // to_owned()
    let course = "Rust".to_string();
    let new_name = name.replace("Cpp", "C++");
    println!("{name} {new_name} {course}");

    let rust = "\x52\x75\x73\x74"; // \x52 = R, \x75 = u, \x73 = s, \x74 = t
    println!("{rust}");

    // String &str
    let color = "res".to_string();
    let name = "John".to_string();
    let person = Person {
        name,
        color,
        age: 30,
    };
    println!("{} is {} and is {} years old", person.name, person.color, person.age);

    // func
    let value = "value".to_owned();
    print(&value);
    print("data");
    // print_string("data"); // 错误：传递 &str 不能调用 String 的函数
    print_string(value);
    // println!("value {} is moved", value); // 错误：value 的所有权已被移动，不能再使用
}

struct Person {
    name: String,
    color: String,
    age: i32,
}

// 可以传递 &str 或 String，不会move所有权
fn print(data: &str) {
    println!("{}", data);
}

// 只能传递 String，而且会move所有权
fn print_string(data: String) {
    println!("{}", data);
}