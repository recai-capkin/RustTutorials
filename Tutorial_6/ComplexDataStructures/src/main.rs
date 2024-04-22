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
    
    let user = createUser();
    println!("User created: {}, {}, ID: {}, College: {}, High School: {}", 
        user.name, user.surname, user.id, user.Education.College, user.Education.High_School);
    
}
fn createUser()-> User {
    let id= read_input("Lütfen id girin: ").parse::<u32>().expect("ID geçerli bir tam sayı değil");
    let name = read_input("Lütfen ad girin: ");
    let surname= read_input("Lütfen soyad girin: ");
    let Education = createEducation();
    return User{ id:id,name:name,surname:surname,Education:Education };
}
fn createEducation() ->Education{
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