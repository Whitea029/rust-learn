static mut MY_STATIC: usize = 100;
static MY_STATIC2: usize = 200;

fn main() {
    const SECOND_HOUR: usize = 60 * 60;
    const SECOND_DAY: usize = 24 * SECOND_HOUR;

    {
        const SECOND_MINUTE: usize = 60;
        println!("一分钟有 {} 秒", SECOND_MINUTE);
    }

    println!("一小时有 {} 秒", SECOND_HOUR);
    println!("一天有 {} 秒", SECOND_DAY);

    unsafe {
        let val = MY_STATIC;
        println!("静态变量 MY_STATIC 的值是: {}", val);

        MY_STATIC += 1;

        let new_val = MY_STATIC;
        println!("修改后的静态变量 MY_STATIC 的值是: {}", new_val);
    }
}