use std::collections::HashMap;
use std::io;
use std::process;

enum CommandType {
    ADD,
    SHOW,
    EXIT,
    CREATE,
    MOVE,
    UNKNOWN,
}

fn handle_messages(message: &String, company: &mut HashMap<String, Vec<String>>) {
    let commands = split_message_to_commands(message);
    match handle_command_type(&commands[0]) {
        CommandType::ADD => handle_add_message(&commands, company),
        CommandType::SHOW => handle_show(&commands, company),
        CommandType::EXIT => process::exit(0),
        CommandType::MOVE => handle_move(&commands, company),
        CommandType::CREATE => handle_create(&commands, company),
        CommandType::UNKNOWN => handle_unkown(message),
    };
}

fn handle_unkown(message: &String) -> bool {
    println!(
        "command uknown - {}\nUsage: [OPTION][VAIRABLES]",
        message.trim()
    );
    true
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
        "exit" | "close" | "quit" => CommandType::EXIT,
        "create" => CommandType::CREATE,
        "move" => CommandType::MOVE,
        _ => return CommandType::UNKNOWN,
    }
}

fn handle_move(commands: &Vec<String>, company: &mut HashMap<String, Vec<String>>) -> bool {
    if commands.len() != 4 || commands[2] != "to" {
        println!("Failed\nUsage: move [EMPLOYEE] to [DEPARTMENT]");
        return false;
    }

    let employee = &commands[1];
    let department = &commands[3];
    if !valid_department_exist(company, department) {
        println!("Failed!{} department not exist", department);
        return false;
    }
    if valid_employee_not_exist(company, employee) {
        println!("Failed!{} employee not exist", department);
        return false;
    }

    remove_employee_department(employee, department, company);
    add_employee_to_department(company, department, employee);
    return true;
}

fn remove_employee_department(
    employee: &String,
    department: &String,
    company: &mut HashMap<String, Vec<String>>,
) -> bool {
    for (_, emp_vec) in company {
        // search for current employee position for deletion
        let mut pos = 0;
        let cuurent_departmt_pos = department.len() - 1;
        while pos <= cuurent_departmt_pos {
            if &emp_vec[pos] == employee {
                emp_vec.remove(pos);
                return true;
            }
            pos += 1;
        }
    }
    false
}

fn handle_create(commands: &Vec<String>, company: &mut HashMap<String, Vec<String>>) -> bool {
    if commands.len() != 2 {
        println!("Failed\nUsage: create [DEPARTMENT]");
    } else {
        let department = &commands[1];
        company.insert(department.to_string(), Vec::new());
        println!("{} is created", department);
    }
    true
}

fn handle_show(commands: &Vec<String>, company: &mut HashMap<String, Vec<String>>) -> bool {
    match commands.len() {
        1 => println!("{:#?}", company),
        2 => show_department(&commands[1], company),
        _ => {
            println!("Failed\nUsage: show [DEPARTMENT] show");
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
    if commands.len() != 4 || commands[2] != "to" {
        println!("Failed\nUsage: add [EMPLOYEE] to [DEPARTMENT]");
        return false;
    }
    let employee = &commands[1].to_lowercase();
    let department = &commands[3].to_lowercase();

    if !valid_department_exist(company, department) {
        println!("Failed\n{} department is not exist", department);
        println!("to create department Usage: create [DEPARTMENT]");
        println!("Usage: add [EMPLOYEE] to [DEPARTMENT]");
        return false;
    } else if !valid_employee_not_exist(company, employee) {
        println!("Failed\n{} is in another department", employee);
        println!("to switch department Usage: move [EMPLOYEE] to [DEPARTMENT]");
        println!("Usage: add [EMPLOYEE] to [DEPARTMENT]");
        return false;
    }
    add_employee_to_department(company, department, employee);
    true
}

fn add_employee_to_department(
    company: &mut HashMap<String, Vec<String>>,
    department: &String,
    employee: &String,
) {
    company
        .entry(department.to_string())
        .or_insert(Vec::new())
        .push(employee.to_string());
    println!("{} added to {} department", employee, department);
}

fn valid_department_exist(company: &HashMap<String, Vec<String>>, department: &String) -> bool {
    match company.get(department) {
        Option::None => {
            return false;
        }
        _ => (),
    }
    true
}

fn valid_employee_not_exist(company: &HashMap<String, Vec<String>>, employee: &String) -> bool {
    for (_, employees) in company {
        for exist_employee in employees {
            if exist_employee == employee {
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
    println!("welcome to HR program");
    loop {
        println!("please enter command");
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        handle_messages(&message, &mut company);
    }
}
