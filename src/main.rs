mod db_operations;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => list(),
        2..=4 => process_command(&args[1], &args.split_at(2).1.to_vec()),
        _ => println!("Too many arguments")
    }
}

fn process_command(flag: &String, arguments: &Vec<String>) {
    match flag.as_str() {
        "-h" => print_help(),
        "-r" => read(arguments),
        "-w" => write(arguments),
        "-W" => overwrite(arguments),
        "-d" => delete(arguments),
        _ => println!("Unrecognized flag {}", flag)
    }
}

// Actual commands
fn list() {
    db_operations::list();
}

fn read(arguments: &Vec<String>) {
    if arguments.len() == 1 {
        db_operations::read_sticky_note(&arguments[0])
    } else {
        println!("'-r' expects one argument (name of sticky note to read).")
    }
}

fn write(arguments: &Vec<String>) {
    if arguments.len() == 2 {
        db_operations::write_sticky_note(&arguments[0], &arguments[1], false)
    } else {
        println!("'-w' expects two arguments (name of sticky note to write, and the text to write).")
    }
}

fn overwrite(arguments: &Vec<String>) {
    if arguments.len() == 2 {
        db_operations::write_sticky_note(&arguments[0], &arguments[1], true)
    } else {
        println!("'-W' expects two arguments (name of sticky note to write, and the text to write).")
    }
}

fn delete(arguments: &Vec<String>) {
    if arguments.len() == 1 {
        db_operations::delete_sticky_note(&arguments[0])
    } else {
        println!("'-d' expects one argument (name of sticky note to delete).")
    }
}

fn print_help() {
    println!("Supported arguments for 'sn':

    (no arguments)   | Lists all sticky note names and a brief preview of their contents.
    -r <name>        | Prints the full sticky note with the given name.
    -w <name> <text> | Writes a sticky note with the given name - no-ops if a note with the given the name already exists.
    -W <name> <text> | Writes a sticky note with the given name - overwrites if a note with the given the name already exists.
    -d <name>        | Deletes the note with the given name.
    -h               | Print this help.")
}
