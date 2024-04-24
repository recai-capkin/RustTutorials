use std::io;
struct User{
    id:u32,
    name:String,
    surname:String,
    Education:Education
}
struct Education{
    id:u32,
    College:String,
    High_School:String
}
fn main() {
    let mut user_list = Vec::new();
    let count = read_input("Kaç kullanıcı ekleyeceksiniz? ").parse::<u32>().unwrap_or(0);
    for _ in  0..count{
        println!("Creating user {}", user_list.len() + 1);
        user_list.push(create_user())
    }

    for User in user_list {
        println!("User created: {}, {}, ID: {}, College: {}, High School: {}", 
        User.name, User.surname, User.id, User.Education.College, User.Education.High_School);
    }
   
    
}
fn create_user()-> User {
    let id= read_input("Lütfen id girin: ").parse::<u32>().expect("ID geçerli bir tam sayı değil");
    let name = read_input("Lütfen ad girin: ");
    let surname= read_input("Lütfen soyad girin: ");
    let education = create_education();
    return User{ id:id,name:name,surname:surname,Education:education };
}
fn create_education() ->Education{
    //Bu kod Result<u32, ParseIntError> döndürdüğü için bu değeri doğrudan bir u32 bekleyen bir yapıya atamak tip uyuşmazlığına sebep olur.
    //Bu yüzden expect kullanımı mecburidir
    //Eğer parse başarısız olursa program durur. Bu, tip güvenliğini sağlar ve olası bir ParseIntError hatasını ele almamızı sağlar
    //Böylelikle id değeri güvenle u32 olarak kullanılabilir
    let id = read_input("Lütfen id girin: ").parse::<u32>().expect("ID geçerli bir tam sayı değil");
    let college = read_input("Lütfen Üniversite girin: ");
    let High_School = read_input("Lütfen lise giriniz: ");
    return Education{ id:id,College:college,High_School:High_School };
}
fn read_input(propmt: &str) -> String {
    println!("{}",propmt);
    //String kütüphanesinden new metodu çağrılıyor ve string türünde boş bir nesne oluşturuyor.
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Beklenmedik bir hata");
    return input.trim().to_string();
}