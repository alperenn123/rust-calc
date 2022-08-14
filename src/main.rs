use std::io::stdin;

struct UserInput {
    first_input: String,
    second_input: String,
}

fn get_user_input() -> UserInput {
    let mut val1 = String::new();
    let mut val2 = String::new();

    println!("Enter First Number \n");
    stdin().read_line(&mut val1).unwrap();

    println!("Enter Second Number \n");
    stdin().read_line(&mut val2).unwrap();

    UserInput {
        first_input: val1,
        second_input: val2,
    }
}

fn add_operation() {
    let user_input = get_user_input();

    let trimmed_val1 = user_input.first_input.trim().parse::<u32>().unwrap();
    let trimmed_val2 = user_input.second_input.trim().parse::<u32>().unwrap();

    println!("Add Operation Result is {}", trimmed_val2 + trimmed_val1);
}

fn minus_operation() {
    let user_input = get_user_input();

    let trimmed_val1 = user_input.first_input.trim().parse::<u32>().unwrap();
    let trimmed_val2 = user_input.second_input.trim().parse::<u32>().unwrap();

    println!("Add Operation Result is {}", trimmed_val2 - trimmed_val1);
}

fn division_operation() {
    let user_input = get_user_input();

    let trimmed_val1 = user_input.first_input.trim().parse::<f32>().unwrap();
    let trimmed_val2 = user_input.second_input.trim().parse::<f32>().unwrap();

    println!("Add Operation Result is {}", trimmed_val2 / trimmed_val1);
}

fn multiply_operation() {
    let user_input = get_user_input();

    let trimmed_val1 = user_input.first_input.trim().parse::<u32>().unwrap();
    let trimmed_val2 = user_input.second_input.trim().parse::<u32>().unwrap();

    println!("Add Operation Result is {}", trimmed_val2 * trimmed_val1);
}

fn print_menu() {
    println!("Enter Your Operator Choice\n\n");
    println!("+\n");
    println!("-\n");
    println!("/ \n");
    println!("* \n");
    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "+" => {
            add_operation();
        }
        "-" => {
            minus_operation();
        }
        "/" => {
            division_operation();
        }
        "*" => {
            multiply_operation();
        }
        _ => println!("Something else"),
    }
}

fn main() {
    print_menu();
}
