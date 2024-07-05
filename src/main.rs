use csv::{WriterBuilder, ReaderBuilder, Reader, StringRecord, Writer};
use std::{io, process, error::Error, fs::OpenOptions, fs::File};


fn main() {
    which_file();
}

//stores the file of the name and calls for the choice_input func
fn which_file(){
    let final_path: String = input_path_file();
    choice_input(&final_path);
}

fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader: Reader<File> = Reader::from_path(file_path)?;
    println!("----------------------------------------------------------------------------");
    
    //prints the headers
    let headers: &StringRecord = reader.headers()?;
    for header in headers.iter() {
        print!("{} | ", header);
    }
    println!();
    println!("----------------------------------------------------------------------------");
    //prints the other rows
    for result in reader.records() {
        let record: StringRecord = result?;

        for field in record.iter() {
            print!("{}, ", field);
        }
        println!();
    }
    println!("----------------------------------------------------------------------------");
    Ok(())
}

//user writes the data that wants to add and then return the vector new_data of strings 
fn data_to_insert(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut reader: Reader<File> = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    let header_result: &StringRecord = reader.headers()?;


    let mut new_data: Vec<String> = Vec::new();

    //for every header the user must write the input that he wants to add and then it adds it to the vector new_data
    for header in header_result.iter() {
        println!("Write to {}, ", header);

        let mut input_for_header: String = String::new();

        io::stdin().read_line(&mut input_for_header).expect("Failed to read line!");
        let input_for_header: String = input_for_header.trim().to_string();
        new_data.push(input_for_header);

    }

    Ok(new_data)
    
}

//adds a new row at the end with the input from data_to_insert function
fn write_in_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file: File = OpenOptions::new().append(true).open(file_path)?;
    
    let mut writer: Writer<File> = WriterBuilder::new().has_headers(true).from_writer(file);


    let new_data: Vec<String> = data_to_insert(file_path)?;
    writer.write_record(&new_data)?;

    writer.flush()?;

    Ok(())
}

//returns the name of the final path of the csv file
fn input_path_file() -> String{
    //read the name of the csv file from stdin
    let mut file_path: String = String::new();
    println!("Write the name of the csv file: ");
    io::stdin().read_line(&mut file_path).expect("Failed to read line!");

    //concat it with the ./ to find the file
    let file_path: &str = file_path.trim();
    let path: String = String::from("./");
    let final_path: String =  path + &file_path;

    final_path
}

//from specific index of the column and specific string to delete, this function adds everything to a new vector 
//except the string that the user gives of the specific index of the column and then rewrite the csv file
//from the content of the vector with the name new_csv_vec
fn remove_from_csv(file_path: &str, index_for_column: usize, string_to_remove: &str) -> Result<(), Box<dyn Error>> {
    let file: File = File::open(file_path)?;
    
    let mut reader: Reader<File> = ReaderBuilder::new().from_reader(file);

    let mut new_csv_vec: Vec<StringRecord> = vec![];

    for result in reader.records() {
        let record: StringRecord = result?;

        let indx: &str = record.get(index_for_column).ok_or("Invalid index for column!")?;

        if indx.trim() == string_to_remove.trim() {
            continue;
        }
        else {
            new_csv_vec.push(record);
        }
    }

    let header: &StringRecord = reader.headers()?;

    let final_csv_file: File = OpenOptions::new().write(true).truncate(true).open(file_path)?;

    let mut writer: Writer<File> = WriterBuilder::new().from_writer(final_csv_file);

    writer.write_record(header)?;

    for record in new_csv_vec {
        writer.write_record(&record)?;
    }

    writer.flush()?;

    Ok(())
}


//handles the choices depends of the user input
fn choice_handler(choice: usize, final_path: &str) {
    match choice {
        1 => {
            if let Err(e) = read_csv(&final_path){
                eprintln!("{}", e);
            }
            choice_input(&final_path);
        },
        2 => {
            if let Err(e) = write_in_csv(&final_path) {
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

            if let Err(e) = remove_from_csv(&final_path, index_for_column, &string_to_remove) {
                println!("error running example: {e}");
                process::exit(1);  
            }
            choice_input(&final_path);
        },
        4 => {
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
    println!("    1 for print the csv file,");
    println!("    2 for writing a new row of data in the csv file,");
    println!("    3 for removing a row from specific columns,");
    println!("    4 for exit");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line!");

    let choice: usize = choice.trim().parse().expect("Please type a number!");

    choice_handler(choice, &final_path);
}

