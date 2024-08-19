use std::{io, process, error::Error, fs::File};
use csv::{Reader, StringRecord, Writer};


mod simple_system_csv;
mod sort_system_csv;
mod specific_mod_system;

fn main() {
    if let Err(e) = which_file(){
        println!("Error: {}", e);
        process::exit(1);
    }
}

//stores the file of the name and calls for the choice_input func
fn which_file()->Result<(), Box<dyn Error>>{
    let final_path: String = simple_system_csv::input_path_file(0);
    let path_to_save: String = simple_system_csv::input_path_file(1);
    temp_final_path(&final_path, &path_to_save)?;
    choice_input(&path_to_save);
    Ok(())
}


//creates the temp file with the name of the path_to_save and copy all the data from the final_path
fn temp_final_path(final_path: &str, path_to_save: &str) ->Result<(), Box<dyn Error>> {
    let mut rdr: Reader<File> = Reader::from_path(&final_path)?;
    
    let headers: &StringRecord = rdr.headers()?;
    
    let temp_file: File = File::create(path_to_save)?;

    let mut wrt: Writer<File> = Writer::from_writer(temp_file);

    wrt.write_record(headers)?;

    for record in rdr.records() {
        let records: StringRecord = record?;

        wrt.write_record(&records)?;
    }

    wrt.flush()?;
    Ok(())
}


//handles the choices depends of the user input
fn choice_handler(choice: usize, path_to_save: &str) {
    match choice {
        1 => {
            if let Err(e) = simple_system_csv::read_csv(&path_to_save){
                eprintln!("{}", e);
            }
            choice_input(&path_to_save);
        },
        2 => {
            if let Err(e) = simple_system_csv::write_in_csv(&path_to_save) {
                println!("error running example: {e}");
                process::exit(1);
            }
            choice_input(&path_to_save);
        },
        3 => {

            match simple_system_csv::ask_the_user_for_what_to_delete(path_to_save){
                Ok((index_for_column,string_to_remove)) => {
                    let string_to_remove: &str = string_to_remove.trim();

                    if let Err(e) = simple_system_csv::remove_from_csv(&path_to_save, index_for_column, &string_to_remove) {
                        println!("error running example: {e}");
                        process::exit(1);  
                    }
                }
                Err(e) => println!("Error: {}", e)
            }


            choice_input(&path_to_save);
        },
        4 => {
            match sort_system_csv::input_for_sorting(path_to_save) {
                Ok((which_column, ascending, what_type)) =>{

                    if let Err(e) = sort_system_csv::sort_column(&path_to_save, which_column, ascending, &what_type) {
                        println!("error running example: {e}");
                        process::exit(1);  
                    }
                }
                Err(e) => println!("Error: {}", e)
            }
            choice_input(&path_to_save);        

        },
        5 => {
            if let Err(e) = specific_mod_system::specific_modification(path_to_save){
                println!("error running example: {e}");
                process::exit(1);  
            }
            choice_input(&path_to_save);
        },
        6 => {
            let path_to_delete: String = simple_system_csv::input_path_file(3);
            
            if let Err(e) = simple_system_csv::delete_csv_file(&path_to_delete) {
                println!("error running example: {e}");
                process::exit(1); 
            }
            choice_input(&path_to_save);
        },
        7 => {
            println!("Program exit succesfully!");
            process::exit(1);
        },
        _ => {
            println!("Choice: {choice} is not existing, please try again!");
            choice_input(&path_to_save);
        }
    }
}

//asks the user for the input of the choice
fn choice_input(path_to_save: &str) {

    let mut choice: String = String::new();

    println!("Choices:");
    println!("    1 print the csv file,");
    println!("    2 insert,");
    println!("    3 delete,");
    println!("    4 sort,");
    println!("    5 specific values to keep");
    println!("    6 delete file");
    println!("    7 exit");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line!");

    let choice: usize = choice.trim().parse().expect("Please type a number!");

    choice_handler(choice, &path_to_save);
}

