use std::io;
fn main() {
    //Skaler Türler
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    // Tip çıkarımı ile
    let default_float = 3.5;
    let default_integer = 7;
    let mut inferred_type=12;
    inferred_type = 18885888521858;

    // Değişkenler varsayılan olarak immutable'dır. Değiştirilebilir bir değişken için `mut` kullanılmalıdır.
    let mut mutable_variable = 15;
    mutable_variable = 20;

    // Bileşik türler
    let tuple:(i32, f64, u8) = (500, 6.4,1);
    println!("Tuple içeriği: ({}, {}, {})", x, y, z)

    // Dizi
    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // Dizideki elemanlara indeks ile erişmek
    let first = _months[0];
    let second = _months[1];
    println!("Yılın ilk iki ayı: {}, {}", first, second);

    // Dizinin tipi ve boyutu belirtilerek
    let _a:[i32; 5] = [1,2,6,8,9];

    // Aynı değere sahip bir dizi oluşturmak
    let _b = [3; 5]; // [3, 3, 3, 3, 3]
}
