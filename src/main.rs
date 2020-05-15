use std::collections::HashMap;
use std::io;
use std::process;

enum CommandType {
    ADD,
    SHOW,
    EXIT,
    CREATE,
    UNKNOWN,
}

fn handle_messages(message: &String, company: &mut HashMap<String, Vec<String>>) {
    let commands = split_message_to_commands(message);
    match handle_command_type(&commands[0]) {
        CommandType::ADD => {
            handle_add_message(&commands, company);
        }
        CommandType::SHOW => {
            handle_show(&commands, company);
        }
        CommandType::EXIT => {
            process::exit(0);
        }
        CommandType::UNKNOWN => {
            println!("command uknown - {}", message.trim());
            println!("Usage: [OPTION][VAIRABLES]");
        }
    };
}

fn split_message_to_commands(message: &String) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for word in message.split_whitespace() {
        commands.push(word.to_string());
    }
    commands
}

fn handle_command_type(command: &str) -> CommandType {
    match command {
        "add" => return CommandType::ADD,
        "show" => return CommandType::SHOW,
        "exit" | "close" => CommandType::EXIT,
        "create" => CommandType::CREATE,
        _ => return CommandType::UNKNOWN,
    }
}

fn handle_show(commands: &Vec<String>, company: &mut HashMap<String, Vec<String>>) -> bool {
    match commands.len() {
        1 => println!("{:#?}", company),
        2 => show_department(&commands[1], company),
        _ => {
            println!("Failed\nUsage: show [DEPARTMENT] OR show");
            return false;
        }
    }
    true
}

fn show_department(department: &String, company: &mut HashMap<String, Vec<String>>) {
    match company.get(department) {
        Some(dep) => println!("{} - {:?}", department, dep),
        _ => println!(
            "Failed department {} not exist\n Usage: show[DEPARTMENT])",
            department,
        ),
    }
}

fn handle_add_message(commands: &Vec<String>, company: &mut HashMap<String, Vec<String>>) -> bool {
    let employee = &commands[1].to_lowercase();
    let to = &commands[2].to_lowercase();
    let department = &commands[3].to_lowercase();
    if commands.len() != 4 || to != "to" {
        println!("Failed\nUsage: add [EMPLOYEE] to [DEPARTMENT]");
        return false;
    }
    if valid_department_exist(company, department) == false
        || valid_employee_exist(company, employee) == false
    {
        println!("Usage: add [EMPLOYEE] to [DEPARTMENT]");
        return false;
    }

    company
        .entry(department.to_string())
        .or_insert(Vec::new())
        .push(employee.to_string());
    println!("{} added to {} department", employee, department);
    true
}

fn valid_department_exist(company: &HashMap<String, Vec<String>>, department: &String) -> bool {
    match company.get(department) {
        Option::None => {
            println!("Failed\n{} department is not exist", department);
            println!("to create department Usage: create [DEPARTMENT]");
            return false;
        }
        _ => (),
    }
    true
}

fn valid_employee_exist(company: &HashMap<String, Vec<String>>, employee: &String) -> bool {
    for (department, employees) in company {
        for exist_employee in employees {
            if exist_employee == employee {
                println!("Failed\n{} is in {} ", employee, department);
                println!("to switch department Usage: move [EMPLOYEE] to [DEPARTMENT]");
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let mut company = HashMap::new();
    let mut my_vec = Vec::new();
    my_vec.push("avi".to_string());
    company.insert("engineering".to_string(), my_vec);
    loop {
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        handle_messages(&message, &mut company);
    }
}
