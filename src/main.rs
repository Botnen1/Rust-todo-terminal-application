use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;           //imports all the necessary modules from the Rust Standard Libaray

fn main() {
    let args: Vec<String> = env::args().collect();
                                                                    //the main function, which is the entry point for a Rust program
    if args.len() < 2 {
        println!("Please provide an argument: add or view");
        return;
    }                       //This block checks whether any arguments are provided with the command. 
                            //If not, it prints a message asking for arguments and then returns 
                            //from the main function, terminating the program.

    let command = &args[1];
            //This line saves the first argument (the command) into the command variable.


    match command.as_str() {
        "add" => add_task(&args),
        "view" => view_tasks(),
        _ => println!("Unknown command. Please use 'add' or 'view'"),
    }           //this statement checks the command str to a specific function name, if its anything but add or view it gives an error
}

fn add_task(args: &[String]) {
    if args.len() < 3 {
        println!("Please provide a task to add");
        return;
    }

    let task = &args[2];
    let mut file = OpenOptions::new()
        .append(true)
        .create(true) // add this line
        .open("tasks.txt")
        .expect("Failed to open tasks.txt");

    writeln!(file, "{}", task).expect("Failed to write task to file");
}



fn view_tasks() {
    let path = Path::new("tasks.txt");

    if !path.exists() {
        println!("Task list is empty");
        return;
    }

    let file = fs::File::open(path).expect("Failed to open tasks.txt");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Could not read line from file");
        println!("Task {}: {}", index + 1, line);
    }
}
