#[derive(Debug)]
struct MyError {
    message: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom Error: {}", self.message)
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.message
    }
}

fn func() -> Result<(), MyError> {
    Err(MyError {
        message: "An error occurred".to_string(),
    })
}

fn main() {
    match func() {
        Ok(_) => println!("Function executed successfully"),
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
