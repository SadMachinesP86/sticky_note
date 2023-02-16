mod db_operations;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1..=4 => process_command(&args[1], &args.split_at(2).1.to_vec()),
        _ => println!("Too many arguments")
    }
}

fn process_command(command: &String, arguments: &Vec<String>) {
    match command.as_str() {
        "list" => list(),
        "read" => read(arguments),
        "write" => write(arguments),
        "edit" => edit(arguments),
        "delete" => delete(arguments),
        _ => println!("Unrecognized command {}", command)
    }
}

// Actual commands
fn list() {
    db_operations::list();
}

fn read(arguments: &Vec<String>) {
    match arguments.len() {
        0 => println!("'Read' expects an argument (name of sticky note to read)."),
        _ => db_operations::read_sticky_note(&arguments[0])
    }
}

fn write(arguments: &Vec<String>) {
    match arguments.len() {
        0..=1 => println!("'Write' expects two arguments (name of sticky note to write, and the text to write)."),
        _ => db_operations::write_sticky_note(&arguments[0], &arguments[1])
    }
}

fn edit(arguments: &Vec<String>) {
    match arguments.len() {
        0..=1 => println!("'Edit' expects two arguments (name of sticky note to edit, and the text to write)."),
        _ => db_operations::edit_sticky_note(&arguments[0], &arguments[1])
    }
}

fn delete(arguments: &Vec<String>) {
    match arguments.len() {
        0 => println!("'Delete' expects an argument (name of sticky note to delete)."),
        _ => db_operations::delete_sticky_note(&arguments[0])
    }
}
