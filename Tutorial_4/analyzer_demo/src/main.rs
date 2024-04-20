use std::io;

fn main() {
    // Kullanıcıya bir sayı girmesini söyleyelim.
    println!("Lütfen bir sayı giriniz:");

    // Burada input adında değiştirilebilir (mutable) bir String nesnesi oluşturulur. 
    // String::new() fonksiyonu, yeni ve boş bir string döndürür. Bu string, kullanıcıdan alınacak girdiyi saklamak için kullanılacaktır.
    let mut input = String::new();

    // Kullanıcıdan girdiyi okuyoruz.
    io::stdin()
        .read_line(&mut input)
        .expect("Girdi okunamadı!");

   
    //input.trim(): trim metodu, input stringinin başındaki ve sonundaki boşlukları veya yeni satır karakterlerini kaldırır. 
    //Bu, kullanıcının girdiğinde oluşabilecek yanlışlıkla eklenen boşlukları temizlemek için kullanılır.

    //parse(): String tipindeki input değişkenini parse ederek bir sayıya (i32) dönüştürmeyi dener. Bu işlem başarılı olursa Ok(T) 
    //(burada T, başarılı bir şekilde parse edilen sayıdır), başarısız olursa Err(E) (burada E, hata bilgisini içeren bir türdür) içeren Result tipi bir değer döndürür.

    //Result<i32, _>: Bu ifade, parse metodunun bir Result tipi döndüreceğini belirtir. i32,
    // başarılı parse işleminin sonucunun türüdür (32-bit tam sayı). _ işareti ise hata tipinin ne olduğu konusunda belirsiz olunduğunu ifade eder
    // ve Rust derleyicisinin hata tipini kendi çıkarımına bırakır.
    let number: Result<i32, _> = input.trim().parse();

    //Bu satır match ifadesiyle number değişkeninin değerine göre bir eşleştirme yapılmasını başlatır.
    // match ifadesi Rust'ta çok güçlü bir kontrol yapısıdır ve çeşitli olası durumları temiz bir şekilde ele almanıza olanak tanır.
    match number {
        //Eğer parse işlemi başarılı olduysa, number adında yeni bir değişken (Ok varyantı içindeki değer) oluşturur ve
        // => sonrasındaki bloğun içine girer. Bu blok, Ok varyantının içindeki değeri kullanarak işlemler yapmak için çalıştırılır.
        Ok(number) => {
            // Alınan sayıyı ikiyle çarpıyoruz.
            let result = 2 * number;

            // Sonucu ekrana basıyoruz.
            println!("Girdiğiniz sayının iki katı: {}", result);
        },
        //Eğer parse işlemi bir hata ile sonuçlanmışsa, Err varyantına girer. _ işareti, hata türünün ne olduğuyla ilgilenmediğimizi ve dolayısıyla herhangi 
        //bir hata durumunda aynı aksiyonun alınacağını gösterir. Bu durumda, kullanıcıya "Lütfen geçerli bir sayı giriniz!" mesajı ekrana basılır.
        Err(_) => println!("Lütfen geçerli bir sayı giriniz!"),
    }
    
}
