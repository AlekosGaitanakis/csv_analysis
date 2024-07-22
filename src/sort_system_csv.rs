use csv::{Reader, StringRecord, Writer};
use std::{error::Error, fs::{self, File}, io::{self}, vec};

//asks the user the column that want to sort and then asks the user
//for ascending or descending
pub fn input_for_sorting() -> (usize, bool, String){

    println!("Write the column you want to sort");

    //input for which column to sort
    let mut which_column = String::new();

    io::stdin()
        .read_line(&mut which_column)
        .expect("Failed to read the line");

    println!("Ascending: Write \"true\" (for low to high) or \"false\" (for high to low)");
    
    //input for ascending
    let mut ascending = String::new();
    io::stdin()
        .read_line(&mut ascending)
        .expect("Failed to read the line");


    

    println!("Write \"int\" for integer sorting or \"float\" for float sorting or \"str\" for string sorting");

    //input for the type of column to sort
    let mut what_type = String::new();

    io::stdin()
        .read_line(&mut what_type)
        .expect("Failed to read line");

    let what_type:String = what_type.to_string();
    let which_column: usize = which_column.trim().to_string().parse().expect("Failed to parse");
    let ascending: bool = ascending.trim().to_string().parse::<bool>().expect("Failed to parse");

    (which_column, ascending, what_type)
}


//ascending:
//  true for low to high and false for high to low
pub fn sort_column(path_to_save: &str, which_column: usize, ascending: bool, what_type: &str) -> Result<(), Box<dyn Error>> {

    //data_of_the_column stores the records that they are not empty in the which_column field 
    //empty_vector stores the records that they are empty in the which_column field
    let (mut data_of_the_column, empty_vector) = create_vector_with_the_data_of_the_column(&path_to_save, which_column)?;


    let mut rdr: Reader<File> = Reader::from_path(&path_to_save)?;

    //temp file to save the sorted file because we can't open and write in the same path in the same time
    let sorted_temp_path: &str = "./sorted_temp.csv";

    //add the headers to var
    let headers: &StringRecord = rdr.headers()?;

    let mut wtr: Writer<File> = Writer::from_path(&sorted_temp_path)?;

    //writes headers to soted_temp.csv
    wtr.write_record(headers)?;

    //remove duplicates
    //data_of_the_column.dedup();

    let what_type: &str = what_type.trim();

    //ascending = true for low to high sorting
    //ascending = false for high to low sorting
    if ascending == true {

        //sorting base of the type of the rows of the specific column
        if what_type == "int" {

            data_of_the_column.sort_by(|a, b| {
                let a = a.parse::<isize>().ok();
                let b = b.parse::<isize>().ok();

                match (a, b) {
                    (Some(a), Some(b)) => a.cmp(&b),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            });

        }
        else if what_type == "float" {

            data_of_the_column.sort_by(|a, b| {
                let a = a.parse::<f64>().ok();
                let b = b.parse::<f64>().ok();

                match (a, b) {
                    (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            });

        }
        else if what_type == "str" {
            data_of_the_column.sort_by(|a, b| a.cmp(&b));
        }

    }
    else if ascending == false{

        if what_type == "int" {

            data_of_the_column.sort_by(|a, b| {
                let a = a.parse::<isize>();
                let b = b.parse::<isize>();

                match (a, b) {
                    (Ok(a), Ok(b)) => b.cmp(&a),
                    (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                    (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                    (Err(_), Err(_)) => std::cmp::Ordering::Equal,
                }
            });

        }
        else if what_type == "float" {
            data_of_the_column.sort_by(|a, b| {
                let a = a.parse::<f64>().ok();
                let b = b.parse::<f64>().ok();

                match (a, b) {
                    (Some(a), Some(b)) => b.partial_cmp(&a).unwrap_or(std::cmp::Ordering::Equal),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            });
        }
        else if what_type == "str" {
            data_of_the_column.sort_by(|a, b| b.cmp(&a));
        }

    }
    //adds the records to the sorted_temp.csv for both vectors 
    for i in &data_of_the_column {
        add_sorted_vector_to_file(which_column, &i, &mut wtr, &path_to_save)?;
    }
    
    for i in &empty_vector {
        add_sorted_vector_to_file(which_column, &i, &mut wtr, &path_to_save)?;
    }
    
    wtr.flush()?;
    
    //adds the sorted file from the sorted_temp.csv file to the file that the user wrote in the start
    fs::copy(sorted_temp_path, path_to_save)?;
    fs::remove_file(sorted_temp_path)?;
    Ok(())
}



//the function checks if any record with the field in the specific column
//contains the specific string and if it exists, writes the record to the sorted_temp.csv file
fn add_sorted_vector_to_file(which_column: usize, elem: &String, wtr: &mut Writer<File>, path_to_save: &str) ->Result<(), Box<dyn Error>> {
    //temp file for sorting
    let temp_file = "temp_file_for_sorting.csv";
    fs::copy(path_to_save, temp_file)?;

    let mut wtr_temp: Writer<File> = Writer::from_path(temp_file)?;

    let mut rdr: Reader<File> =  Reader::from_path(path_to_save)?;

    let headers: &StringRecord = rdr.headers()?;

    wtr_temp.write_record(headers)?;

    let mut counter = 0;
    for record in rdr.records() {
        let records: StringRecord = record?;
        
        if let Some(field) = records.get(which_column) {
            if elem == &field.to_string() && counter == 0{
                wtr.write_record(&records)?;
                counter = counter + 1;
            }
            else {
                wtr_temp.write_record(&records)?;
            }
        }
    }    
    wtr_temp.flush()?;

    fs::copy(temp_file, path_to_save)?;
    fs::remove_file(temp_file)?;

    Ok(())
}


//returns a new vector with all the elements from the field in column that the user gave
fn create_vector_with_the_data_of_the_column(path_to_save: &str, which_column: usize) ->Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let mut rdr: Reader<File> = Reader::from_path(&path_to_save)?;


    let mut sort_vector: Vec<String> = vec![];
    let mut empty_vector: Vec<String> = vec![];

    for record in rdr.records() {
        let records: StringRecord = record?;

        if let Some(field) = records.get(which_column) {
            if field.trim().to_string() == "" {
                empty_vector.push(field.to_string());
            }
            else {
                sort_vector.push(field.to_string());
            }
        }

    }
    Ok((sort_vector, empty_vector))
}

