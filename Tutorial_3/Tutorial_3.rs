use std::io; // Girdi/çıktı işlemleri için standart kütüphane

fn main() {
    println!("Lütfen kullanıcı adınızı girin:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Kullanıcı adı okunamadı.");

    println!("Lütfen parolanızı girin:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Parola okunamadı.");

    // Kullanıcı adı ve parola kontrolü
    if login(&username.trim(), &password.trim()) {
        println!("Giriş başarılı!");
    } else {
        println!("Hatalı kullanıcı adı veya parola!");
    }
}

// Kullanıcı adı ve parola doğrulaması yapan fonksiyon
fn login(username: &str, password: &str) -> bool {
    username == "admin" && password == "12345"
}
