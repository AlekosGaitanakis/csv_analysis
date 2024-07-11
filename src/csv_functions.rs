use csv::{WriterBuilder, ReaderBuilder, Reader, StringRecord, Writer};
use std::{error::Error, fs::{self, File, OpenOptions}, io::{self}};



pub fn delete_csv_file(path_to_delete: &str) -> Result<(),  Box<dyn Error>> {
    fs::remove_file(path_to_delete)?;

    Ok(())
}

fn number_of_minus_to_print(path_to_save: &str) ->Result<(), Box<dyn Error>>{
    let mut rdr: Reader<File> = Reader::from_path(path_to_save)?;

    let headers: &StringRecord = rdr.headers()?;

    for header in headers.iter() {
        for _i in header.chars() {
            print!("-");
        }
    }
    
    println!("----------");
    Ok(())
}

pub fn read_csv(path_to_save: &str) -> Result<(), Box<dyn Error>> {
    let mut reader: Reader<File> = Reader::from_path(path_to_save)?;
    number_of_minus_to_print(&path_to_save)?;
    
    //prints the headers
    let headers: &StringRecord = reader.headers()?;
    for header in headers.iter() {
        print!("{} | ", header);
    }
    println!();
    number_of_minus_to_print(&path_to_save)?;
    //prints the other rows
    for result in reader.records() {
        let record: StringRecord = result?;

        for field in record.iter() {
            print!("{}, ", field);
        }
        println!();
    }
    number_of_minus_to_print(&path_to_save)?;
    Ok(())
}

//user writes the data that wants to add and then return the vector new_data of strings 
pub fn data_to_insert(path_to_save: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut reader: Reader<File> = ReaderBuilder::new().has_headers(true).from_path(path_to_save)?;

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

//ask the user for the index that he wants to delete and then ask him what to delete
pub fn ask_the_user_for_what_to_delete() ->(usize, String){
    println!("Write the index of the column(starts from 0) :");
    let mut index_for_column: String = String::new();

    io::stdin()
        .read_line(&mut index_for_column)
        .expect("Failed to read line!");
        
    let index_for_column: usize = index_for_column.trim().parse().expect("Reason");
            
    println!("Write what you want to remove from the column that you wrote before :");
            
    let mut string_to_remove: String = String::new();
    io::stdin()
        .read_line(&mut string_to_remove)
        .expect("Failed to read the line!");

    let string_to_remove: String = string_to_remove;

    (index_for_column, string_to_remove)
}

//adds a new row at the end with the input from data_to_insert function
pub fn write_in_csv(path_to_save: &str) -> Result<(), Box<dyn Error>> {
    let file: File = OpenOptions::new().append(true).open(path_to_save)?;
    
    let mut writer: Writer<File> = WriterBuilder::new().has_headers(true).from_writer(file);


    let new_data: Vec<String> = data_to_insert(path_to_save)?;
    writer.write_record(&new_data)?;

    writer.flush()?;

    Ok(())
}

//returns the name of the final path of the csv file
pub fn input_path_file(what_to_do: usize) -> String{
    //read the name of the csv file from stdin
    let mut path_to_save: String = String::new();
    if what_to_do == 0 {
        println!("Write the name of the csv file to open: ");
        io::stdin().read_line(&mut path_to_save).expect("Failed to read line!");
    }
    else if what_to_do == 1 {
        println!("Write the name of the file to save the modificated csv file: ");
        io::stdin().read_line(&mut path_to_save).expect("Failed to read line!");
    }
    else if what_to_do == 3 {
        println!("Write the name of the file to delete: ");
        io::stdin().read_line(&mut path_to_save).expect("Failed to read line!");   
    }


    //concat it with the ./ to find the file
    let path_to_save: &str = path_to_save.trim();
    let path: String = String::from("./");
    let path_to_save: String =  path + &path_to_save + ".csv";

    path_to_save
}

//from specific index of the column and specific string to delete, this function adds everything to a new vector 
//except the string that the user gives of the specific index of the column and then rewrite the csv file
//from the content of the vector with the name new_csv_vec
pub fn remove_from_csv(path_to_save: &str, index_for_column: usize, string_to_remove: &str) -> Result<(), Box<dyn Error>> {
    let file: File = File::open(path_to_save)?;
    
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

    let final_csv_file: File = OpenOptions::new().write(true).truncate(true).open(path_to_save)?;

    let mut writer: Writer<File> = WriterBuilder::new().from_writer(final_csv_file);

    writer.write_record(header)?;

    for record in new_csv_vec {
        writer.write_record(&record)?;
    }

    writer.flush()?;

    Ok(())
}

