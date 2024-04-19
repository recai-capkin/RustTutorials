const PI: f64 = 3.141592653589793238;
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: i32 = 0;
fn main() {
    println!("Pi değeri: {}", PI);
    unsafe {
        COUNTER += 1; // Bu şekilde bir static mutable değişkene güvenli olmayan bir blok içinde erişilir.
        println!("COUNTER: {}", COUNTER);
    }
    println!("{}", HELLO_WORLD);
}



