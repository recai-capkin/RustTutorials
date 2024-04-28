const MAX_POINTS: u32 = 100_000; // Sabit bir değer tanımlama
fn main() {
    let x: i32 = 2147483647; // Değiştirilemez bir değişken tanımlama
    println!("Değişkenin değeri: {}", x);

    println!("Maksimum puan: {}", MAX_POINTS);

    let mut y = 5; // Değiştirilebilir bir değişken tanımlama
    println!("Değişkenin ilk değeri: {}", y);
    y = 6; // y değişkeninin değerini değiştirebiliriz.
    println!("Değişkenin yeni değeri: {}", y);

    unsafe {
        // Ham işaretçilerle çalışma
        let mut num = 5;
    
        let r1 = &num as *const i32; // num'a sabit işaretçi
        let r2 = &mut num as *mut i32; // num'a değiştirilebilir işaretçi

        *r2 = 15;
    
        println!("r1 işaret ettiği değer: {}", *r1);
        println!("r2 işaret ettiği değer: {}", *r2);
        
        println!("num işaret ettiği değer: {}", num);
        // Dış C fonksiyonunu çağırma
        extern "C" {
            fn abs(input: i32) -> i32;
        }
    
        println!("C'nin abs fonksiyonu: {}", abs(-3));  // C standard kütüphanesinden bir fonksiyon
    }
    unsafe {
        let mut data = vec![1, 2, 3, 4];
        let data_ptr = data.as_mut_ptr();  // data_ptr, heap'teki vektörün ilk elemanına işaret eder
    
        // data_ptr üzerinden ilk elemanı direkt olarak değiştirme
        *data_ptr = 10;
    
        println!("Güncellenmiş vektör: {:?}", data);
    }
}
