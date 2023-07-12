    use std::io;

    fn main() {
        println!("Simple Calculator");
        println!("Enter your first number: ");
        let mut num1 = String::new();
        let mut num2 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed! Now debug noob..");
        println!("Enter your second number: ");
        io::stdin().read_line(&mut num2).expect("Failed! Now debug noob.");
        let num1: f64 = num1.trim().parse().expect("Enter a number dude!");
        let num2: f64 = num2.trim().parse().expect("Enter a number dude!");

        println!("Select an operation");
        println!("1. Addition");
        println!("2. Division");
        println!("3. Multiplication");
        println!("4. Subtraction");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed! Now debug noob.");
        let choice: u32 = choice.trim().parse().expect("Invalid choice!");
        let result = match choice {
            1 => num1 + num2,
            2 => {
                if num2 == 0.0 {
                    panic!("Division by zero error");
                } else {
                    num1 / num2
                }
            },
            3 => num1 * num2,
            4 => num1 - num2,
            _ => panic!("Invalid operation"),
            3 => num1 * num2,
            4 => num1 - num2,
            _ => panic!("Invalid operation"),
        };
        println!("Result: {}", result);
    }
