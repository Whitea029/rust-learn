enum Color {
    Res,
    Yellow,
    Blue,
    Black,
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Res => println!("The color is Red"),
        Color::Yellow => println!("The color is Yellow"),
        Color::Blue => println!("The color is Blue"),
        Color::Black => println!("The color is Black"),
    }
}

enum BuildingLocation {
    Number(i32),
    Name(String),
    Unknown,
}

impl BuildingLocation {
    fn print_location(&self) {
        match self {
            BuildingLocation::Number(num) => println!("Building number: {}", num),
            BuildingLocation::Name(name) => println!("Building name: {}", name),
            BuildingLocation::Unknown => println!("Building location is unknown"),
        }
    }
}

fn main() {
    print_color(Color::Black);

    let house = BuildingLocation::Name(String::from("House"));

}
