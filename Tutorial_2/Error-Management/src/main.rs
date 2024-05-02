mod result_management;
mod options_management;
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
}
