use arch::Mode;
use dog_face::DogMode;
use utils::input;

fn main() {
    println!("Welcome!");
    loop { 
        let command = input("(mag) ").unwrap();
        println!("{}",command);
        match command.as_str() {
            "dog" => {
                let mut mode = DogMode::new();
                mode.start();
            }
            "exit" | "q" | "c" | "quit"  => {
                break;
            }
            _ => {}
        }
    }
}
