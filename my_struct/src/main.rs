fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 2.50,
    };
    println!("{}", sweet.price);
    print_drink(sweet);
    let sour = Drink::new(Flavor::Sour, 3.00);
    sour.buy();
    
    // ownership
    let mut c1 = Counter::new(0);
    println!("Counter 1: {}", c1.get_number());
    c1.add(2);
    println!("Counter 1 after adding 2: {}", c1.get_number());
    c1.give_up();
    // println!("Counter 1 after giving up: {}", c1.get_number());
    let c1 = Counter::new(1);
    let c2 = Counter::new(2);
    let c3 = Counter::combine(c1, c2);
    println!("Counter 3: {}", c3.get_number());
    // println!("Counter 1 after combining: {}", c1.get_number());
    // println!("Counter 2 after combining: {}", c2.get_number());
}

struct Drink {
    flavor: Flavor,
    price: f64,
}

impl Drink {
    // 关联变量
    const MAX_PRICE: f64 = 10.0;
    // 不可变方法
    fn buy (&self) {
        if self.price > Drink::MAX_PRICE {
            println!("This drink is too expensive!");
        } else {
            println!("You bought a drink for ${:.2}", self.price);
        }
    }
    // 关联方法
    fn new(flavor: Flavor, price: f64) -> Drink {
        Drink { flavor, price }
    }
}

enum Flavor {
    Spicy,
    Sweet,
    Sour,
}
 
fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Spicy => println!("Drink is spicy and costs ${:.2}", drink.price),
        Flavor::Sweet => println!("Drink is sweet and costs ${:.2}", drink.price),
        Flavor::Sour => println!("Drink is sour and costs ${:.2}", drink.price),
    }
}

struct Counter {
    number: i32,
}

impl Counter {
    fn new(number: i32) -> Self {
        Self { number: number }
    }

    // 不可变借用
    // Counter::get_number(self: &Self) -> i32
    fn get_number(&self) -> i32 {
        self.number
    }

    // 可变借用
    // Counter::add(self: &mut Self, value: i32)
    fn add(&mut self, value: i32) {
        self.number += value;
    }

    // move
    fn give_up(self) {
        println!("Giving up on the counter with value: {}", self.number);        
    }

    // move
    fn combine(c1: Self, c2: Self) -> Self {
        Self {
            number: c1.number + c2.number,
        }
    }
}