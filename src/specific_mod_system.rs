use csv::{Reader, Writer, StringRecord};
use std::{error::Error, fs::{self, File}, io::{self}, isize};


fn ask_the_user_what_specific_modification_needs() -> Result<(usize, String, String), Box<dyn Error>>{

    println!("Write the column you want to do the specific modifications :");

    let mut which_column = String::new();

    io::stdin()
        .read_line(&mut which_column)
        .expect("Failed to read line!");

    println!("Write the type of the data in the column(int, float, string)");
    let mut what_type: String = String::new();

    io::stdin()
        .read_line(&mut what_type)
        .expect("Failed to read the line!");

    let mut what_to_do_string: String = String::new();

    println!("Write what you want to do.\nYou can use <, >, ==, !=, >=, <=.");
    println!("You can also use the && (for AND) and the || (for OR).");
    println!("For Example, > 65, == \"Hello\" and > 65 && < 80, > 65 || < 80.");
    println!("NOTE: use space.");

    io::stdin()
        .read_line(&mut what_to_do_string)
        .expect("Failed to read line!");

    let which_column: usize = which_column.trim().parse::<usize>()?;

    let what_to_do_string: String = what_to_do_string;
    let what_type: String = what_type.trim().to_string();

    Ok((which_column, what_to_do_string, what_type))
}


fn specific_insert_at_csv<T: PartialOrd + PartialEq>(field_val: T, specific_val: T, what_to_do_symbol: &str) -> bool {

    if what_to_do_symbol == ">" {
        field_val > specific_val
    }
    else if what_to_do_symbol == ">=" {
        field_val >= specific_val
    }
    else if what_to_do_symbol == "<" {
        field_val < specific_val
    }
    else if what_to_do_symbol == "<=" {
        field_val <= specific_val
    }
    else if what_to_do_symbol == "==" {
        field_val == specific_val
    }
    else if what_to_do_symbol == "!=" {
        field_val != specific_val
    }
    else {
        false
    }
}
fn what_to_do_function(path_to_save: &str, what_to_do_vec: Vec<&str>, which_column: usize, wtr: &mut Writer<File>, what_type: &str) -> Result<(), Box<dyn Error>> {

    let mut rdr: Reader<File> = Reader::from_path(&path_to_save)?;

    let headers: &StringRecord = rdr.headers()?;

    wtr.write_record(headers)?;

    for record in rdr.records() {
        let record: StringRecord = record?;
        let what_to_do_symbol: &str = what_to_do_vec[0];

        if let Some(val_from_specific_column) = record.get(which_column){
            
            if what_type == "int" {
                let val_from_specific_column: isize = val_from_specific_column.parse::<isize>()?;
                let specific_val: isize = what_to_do_vec[1].parse::<isize>()?;

                if what_to_do_vec.len() == 2 {
                    let what_to_save: bool = specific_insert_at_csv(val_from_specific_column, specific_val, &what_to_do_symbol);
                    if what_to_save == true {
                        wtr.write_record(&record)?;
                    }
                }
                else if what_to_do_vec.len() == 5 {

                    let what_to_do_symbol_second: &str = what_to_do_vec[3];
                    let second_specific_val: isize = what_to_do_vec[4].parse::<isize>()?;
                    let specific_or_or_and: &str = what_to_do_vec[2];

                    let what_to_save_first: bool = specific_insert_at_csv(val_from_specific_column, specific_val, &what_to_do_symbol);
                    let what_to_save_second: bool = specific_insert_at_csv(val_from_specific_column, second_specific_val, &what_to_do_symbol_second);


                    if specific_or_or_and == "&&" {
                        if what_to_save_first == true && what_to_save_second == true{
                            wtr.write_record(&record)?;
                        }
                    }
                    else if specific_or_or_and == "||" {
                        if what_to_save_first == true || what_to_save_second == true {
                            wtr.write_record(&record)?;
                        }
                    }

                }

            }
            else if what_type == "float" {
                let val_from_specific_column: f64 = val_from_specific_column.parse::<f64>()?;
                let specific_val: f64 = what_to_do_vec[1].parse::<f64>()?;

                if what_to_do_vec.len() == 2 {
                    let what_to_save: bool = specific_insert_at_csv(val_from_specific_column, specific_val, &what_to_do_symbol);
                    if what_to_save == true {
                        wtr.write_record(&record)?;
                    }
                }
                else if what_to_do_vec.len() == 5 {

                    let what_to_do_symbol_second: &str = what_to_do_vec[3];
                    let second_specific_val: f64 = what_to_do_vec[4].parse::<f64>()?;
                    let specific_or_or_and: &str = what_to_do_vec[2];

                    let what_to_save_first: bool = specific_insert_at_csv(val_from_specific_column, specific_val, &what_to_do_symbol);
                    let what_to_save_second: bool = specific_insert_at_csv(val_from_specific_column, second_specific_val, &what_to_do_symbol_second);


                    if specific_or_or_and == "&&" {
                        if what_to_save_first == true && what_to_save_second == true{
                            wtr.write_record(&record)?;
                        }
                    }
                    else if specific_or_or_and == "||" {
                        if what_to_save_first == true || what_to_save_second == true {
                            wtr.write_record(&record)?;
                        }
                    }

                }
            }
            else if what_type == "string" {
                let val_from_specific_column: &str = val_from_specific_column;
                let specific_val: &str = what_to_do_vec[1];

                if what_to_do_vec.len() == 2 {
                    let what_to_save: bool = specific_insert_at_csv(val_from_specific_column, specific_val, &what_to_do_symbol);
                    if what_to_save == true {
                        wtr.write_record(&record)?;
                    }
                }
                else if what_to_do_vec.len() == 5 {

                    let what_to_do_symbol_second: &str = what_to_do_vec[3];
                    let second_specific_val: &str = what_to_do_vec[4];
                    let specific_or_or_and: &str = what_to_do_vec[2];

                    let what_to_save_first: bool = specific_insert_at_csv(val_from_specific_column, specific_val, &what_to_do_symbol);
                    let what_to_save_second: bool = specific_insert_at_csv(val_from_specific_column, second_specific_val, &what_to_do_symbol_second);


                    if specific_or_or_and == "&&" {
                        if what_to_save_first == true && what_to_save_second == true{
                            wtr.write_record(&record)?;
                        }
                    }
                    else if specific_or_or_and == "||" {
                        if what_to_save_first == true || what_to_save_second == true {
                            wtr.write_record(&record)?;
                        }
                    }

                }
            }


        }



    }

    
    Ok(())
}

pub fn specific_modification(path_to_save: &str) ->Result<(), Box<dyn Error>> {

    let (which_column, what_to_do_string, what_type) = ask_the_user_what_specific_modification_needs()?;
    
    let what_to_do_vec: Vec<&str> = what_to_do_string.split_whitespace().collect();

    if what_to_do_vec.len() != 2 && what_to_do_vec.len() != 5 {
        specific_modification(&path_to_save)?;
    }

    let specific_modified_path: &str = "./specific_modified_temp.csv";
    let mut wtr: Writer<File> = Writer::from_path(&specific_modified_path)?;
    

    what_to_do_function(path_to_save, what_to_do_vec, which_column, &mut wtr, &what_type)?;

    wtr.flush()?;
    fs::copy(&specific_modified_path, &path_to_save)?;
    fs::remove_file(&specific_modified_path)?;
    Ok(())
    
}