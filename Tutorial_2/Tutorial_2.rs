// const PI: f64 ifadesi, PI isminde bir sabit tanımlar ve bu sabitin tipini f64 olarak belirtir, yani çift hassasiyetli kayan nokta sayı.
const PI: f64 = 3.141592653589793238;

// Programın Ömrü: static değişkenler, program başladığı andan itibaren sonlandığı ana kadar yaşarlar. 
// Bu, onların değerlerinin uygulama boyunca sabit kaldığı ve herhangi bir fonksiyon tarafından erişilebileceği anlamına gelir.
// Sabitleme (Immutability): Rust'ta static değişkenler varsayılan olarak değiştirilemezdir (immutable).
//  Bu, bir kez tanımlandıktan sonra değerlerinin değiştirilemeyeceği anlamına gelir. 
//  Eğer bir static değişkenin değiştirilebilir olmasını istiyorsanız, mut anahtar kelimesini kullanarak bunu belirtmeniz gerekir 
//  (örneğin, static mut VARIABLE: Type = value;).
// Güvenlik: static mut ile tanımlanmış değiştirilebilir statik değişkenlere erişim yalnızca unsafe bloklar içinde yapılabilir,
//  çünkü bu tür erişimler yarış koşulları (race conditions) gibi hatalara yol açabilir.
//   Rust'ın bu kısıtlaması, yarış koşullarının ve diğer bellek güvenliği sorunlarının önlenmesine yardımcı olur.
// Bellek Yönetimi: static değişkenler heap yerine sabit bellek bölgesinde depolanır ve
//  Rust'ın sahiplik (ownership) ve ödünç alma (borrowing) kurallarından muaf tutulurlar. 
//  Bunun sonucu olarak, static değişkenlerin yaşam süreleri manuel olarak yönetilmez.
static HELLO_WORLD: &str = "Hello, world!";

// static mut COUNTER: i32 ifadesi COUNTER adında mutable (değiştirilebilir) bir statik değişken tanımlar ve bu değişkenin tipini i32 olarak belirtir.
// = 0; ifadesi ile bu değişkene başlangıç değeri olarak 0 atanır.
//  mut anahtar kelimesi kullanılarak değiştirilebilir kılınmıştır, ancak bu değişkene erişim yalnızca unsafe bloklar içinde gerçekleşebilir.
static mut COUNTER: i32 = 0;

fn main() {
    println!("Pi değeri: {}", PI);
    // unsafe anahtar kelimesi ile başlayan bu blok, 
    // Rust'ın güvenlik garantilerinden feragat edilen bir kod bloğudur. Burada COUNTER statik mutable değişkeninin değeri artırılıyor.
    unsafe {
        COUNTER += 1; // Bu şekilde bir static mutable değişkene güvenli olmayan bir blok içinde erişilir.
        println!("COUNTER: {}", COUNTER);
    }
    println!("{}", HELLO_WORLD);
}



