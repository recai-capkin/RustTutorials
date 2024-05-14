extern crate anyhow;

mod result_management;
mod options_management;
mod panic_management;
mod error_wrapping_management;

use error_wrapping_management as ewm;
fn main() {
    //Result Management
    let file_path = "example.txt";
    match result_management::read_file_contents(file_path) {
        Ok(contents) => println!("Dosya içeriği: {}", contents),
        Err(e) => println!("Hata: {}", e),
    }

    //Options Management
    options_management::getAges();
    options_management::get_Index();

    //Panic Management
     panic_management::Use(10, 0);

    //Error Wrapping Management
    match ewm::read_file(file_path) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error: {:?}", e),
    }
}
