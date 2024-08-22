use std::fs::File;
// use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
  // let greeting_file_result = File::open("hello.txt");


  // check result and handle errors using match for logic
//   let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         // panics on any error
//         // Err(error) => { panic!("Problem opening the file: {:?}", error) },
        
//         // handle if a file doesn't exist to create it. 
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(error) => panic!("Problem creating the file: {:?}", error),
//             },
//             other_error => panic!("Problem opening the file: {:?}", other_error)
//         }
//     };

    // check and handle with clousures and unwrap_or_else

    // let greeting_file_2 = File::open("hey.txt").unwrap_or_else(|error|{
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hey.txt").unwrap_or_else(|error|{
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });


    // unwrap and expect
    // let greeting_file_3 = File::open("yo.txt").unwrap();
    // let greeting_file_4 = File::open("yo.txt").expect("Failed to open yo.txt");

    // In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.

    let _ = read_username_from_file();
}

fn read_username_from_file() -> Result<String,io::Error> {

    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }




}

// use the ? to simplify the above function
// The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// simplify further
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}