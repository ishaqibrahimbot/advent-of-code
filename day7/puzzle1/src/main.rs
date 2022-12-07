use std::fs;
use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Debug)]
enum CommandType {
    cd,
    ls
}

#[derive(Debug)]
struct Command {
    command_type: Option<CommandType>,
    value: Option<String>,
    output: Option<Vec<String>>
}

#[derive(Clone)]
#[derive(Debug)]
struct File {
    name: String,
    size: u32
}

fn get_commands(lines: Vec<String>) -> Vec<Command> {

    let mut commands: Vec<Command> = Vec::new();
    
    let mut output: Vec<String> = Vec::new();
    let mut command = Command { command_type: None, output: None, value: None };

    for (index, line) in lines.iter().enumerate() {
        if line.starts_with("$") {

            if command.command_type == Some(CommandType::ls) {
                command.output = Some(output);
                commands.push(command);
                command = Command { command_type: None, output: None, value: None };
                output = Vec::new();
            }

            let details: Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
            let command_type = &details[1];


            if command_type == "cd" {
                command.command_type = Some(CommandType::cd);
                command.value = Some(details[2].clone());
                command.output = None;
                commands.push(command);
                command = Command { command_type: None, output: None, value: None };
            } else {
                command.command_type = Some(CommandType::ls);
            }

        } else {
            output.push(line.clone());
        }

        if index == lines.len() - 1 {
            command.output = Some(output);
                commands.push(command);
                command = Command { command_type: None, output: None, value: None };
                output = Vec::new();
        }
    }

    commands
}   

fn process_command(command: Command, path: String, mut list: Vec<File>) -> (String, Vec<File>) {

    if command.command_type == Some(CommandType::cd) {
        if command.value == Some("..".to_string()) {
            let mut directories: Vec<String> = path.split("/").map(|x| String::from(x)).collect();
            println!("Directories before: {:?}", directories);
            directories.pop();
            directories.pop();
            println!("Directories after pop: {:?}", directories);
            let new_path = if directories.len() > 1 { directories.push("".to_string()); directories.join("/") } else { "/".to_string() };
            println!("New path: {}", new_path);
            (new_path, list)
        } else {
            let name_of_dir = command.value.unwrap();
            let mut new_path = path.clone();
            if name_of_dir == "/" {
                new_path.push_str(&name_of_dir);
            } else {
                new_path.push_str(&name_of_dir);
                new_path.push('/');
            }
            (new_path, list)
        }
    } else {
        if let Some(file_or_folder_array)= command.output {
            for file_or_folder in file_or_folder_array {
                if !file_or_folder.starts_with("dir") {
                    let name: Vec<String> = file_or_folder.split(" ").map(|x| String::from(x)).collect();
                    let mut file_name = path.clone();
                    file_name.push_str(&name[1]);
                    let new_file = File {
                        name: file_name,
                        size: name[0].clone().parse().unwrap()
                    };

                    list.push(new_file);
                }
            }
        }
        (path, list)
    }
}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).expect("FAILURE, YOU ARE");

    let lines: Vec<String> = contents.split("\n").map(|x| String::from(x)).collect();

    let commands = get_commands(lines);

    let mut file_list = Vec::new();
    let mut path = String::new();
    for command in commands {
        (path, file_list) = process_command(command, path, file_list);
    }

    let mut total_folder_size_map: HashMap<String, u32> = HashMap::new();

    for file in file_list {

        let mut file_name_parts: Vec<String> = file.name.split("/").map(|x| String::from(x)).collect();
        file_name_parts.pop();

        while file_name_parts.len() > 0 {
            let folder_path = if file_name_parts.len() > 1 { file_name_parts.join("/") } else { "/".to_string() };

            if total_folder_size_map.contains_key(&folder_path) {
                let value = total_folder_size_map.get_mut(&folder_path).unwrap();
                *value += file.size;
            } else {
                total_folder_size_map.insert(folder_path, file.size);
            }

            file_name_parts.pop();
        }

    }

    let mut total_folder_size = 0;

    for size in total_folder_size_map.values() {
        if *size <= 100000 {
            total_folder_size += size;
        }
    }

    println!("FILE TOTALS: {:#?}", total_folder_size_map);
    println!("Total of folders <100000 in size: {total_folder_size}");
}