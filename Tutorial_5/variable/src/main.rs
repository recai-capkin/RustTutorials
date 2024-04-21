use std::io;

static UNMUTE_GLOBAL_VAR: i32 = 10;

//Static programın veri segmentinde saklanır. Bu segment, programın çalıştığı süre boyunca sabit kalan ve değişkenlerin değerlerini tutan bir alandır. 
//Rust, bu tür değişkenleri güvenli bir şekilde yönetmek için katı kurallar koyar çünkü bu değişkenlere eş zamanlı erişim durumunda data race'ler oluşabilir.
static mut MUTE_GLOBAL_VAR: i32 = 100;

const max_limit: i32 = 150;
struct MyStruct {
    field1: i32, // Bu bir struct alanıdır ve MyStruct tipinden bir örneğin parçasıdır.
}
fn main() {
    //Skaler Türler
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    // Tip çıkarımı ile
    let default_float = 3.5;
    let default_integer = 7;
    let mut inferred_type=12;
    inferred_type = 158521858;

    // Değişkenler varsayılan olarak immutable'dır. Değiştirilebilir bir değişken için `mut` kullanılmalıdır.
    let mut mutable_variable = 15;
    mutable_variable = 20;

    // Bileşik türler
    let tuple:(i32, f64, u8) = (500, 6.4,1);
    // Tuple'dan değerleri çıkarmak
    let (x, y, z) = tuple;
    println!("Tuple içeriği: ({}, {}, {})", x, y, z);

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
    unsafe{
        MUTE_GLOBAL_VAR +=5;
        print!("{}",MUTE_GLOBAL_VAR)
    }
    
}
