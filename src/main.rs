use std::{io, process};

pub mod csv_functions;

fn main() {
    which_file();
}

//stores the file of the name and calls for the choice_input func
fn which_file(){
    let final_path: String = csv_functions::input_path_file();
    choice_input(&final_path);
}





//handles the choices depends of the user input
fn choice_handler(choice: usize, final_path: &str) {
    match choice {
        1 => {
            if let Err(e) = csv_functions::read_csv(&final_path){
                eprintln!("{}", e);
            }
            choice_input(&final_path);
        },
        2 => {
            if let Err(e) = csv_functions::write_in_csv(&final_path) {
                println!("error running example: {e}");
                process::exit(1);
            }
            choice_input(&final_path);
        },
        3 => {
            println!("Write the index of the column(starts from 0) :");
            let mut index_for_column: String = String::new();

            io::stdin().read_line(&mut index_for_column).expect("Failed to read line!");
            let index_for_column: usize = index_for_column.trim().parse().expect("Reason");
            
            println!("Write what you want to remove from the column that you wrote before :");
            let mut string_to_remove: String = String::new();
            io::stdin()
                .read_line(&mut string_to_remove)
                .expect("Failed to read the line!");

            let string_to_remove: &str = string_to_remove.trim();

            if let Err(e) = csv_functions::remove_from_csv(&final_path, index_for_column, &string_to_remove) {
                println!("error running example: {e}");
                process::exit(1);  
            }
            choice_input(&final_path);
        },
        // 4 => {
        //     println!("Name of the column");
        //     let mut name_of_column: String = String::new();
        //     io::stdin()
        //         .read_line(&mut name_of_column)
        //         .expect("Failed to read the line!");

        //     let name_of_column: &str = name_of_column.trim();

        //     if let Err(e) = csv_functions::sort_column(&name_of_column, &final_path) {
        //         println!("Error : {}", e);
        //         process::exit(1);
        //     }
        //     choice_input(&final_path);
        // },
        5 => {
            println!("Program exit succesfully!");
            process::exit(1);
        },
        _ => {
            println!("Choice: {choice} is not existing, please try again!");
            choice_input(&final_path);
        }
    }
}

//asks the user for the input of the choice
fn choice_input(final_path: &str) {

    let mut choice: String = String::new();

    println!("Choices:");
    println!("    1 print the csv file,");
    println!("    2 insert,");
    println!("    3 delete,");
    println!("    4 sort,");
    println!("    5 exit");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line!");

    let choice: usize = choice.trim().parse().expect("Please type a number!");

    choice_handler(choice, &final_path);
}

