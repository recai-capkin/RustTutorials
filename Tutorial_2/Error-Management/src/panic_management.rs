fn safe_division(numerator:i32,denominator: i32)-> Result<i32,String>{
    if denominator == 0 {
        Err(String::from("division by zero"))
    }
    else {
        // Bölme işlemi güvenli bir şekilde gerçekleştirilir.
        Ok(numerator / denominator)
    }
}

pub fn Use(numerator:i32,denominator:i32)
{
    let result = safe_division(numerator, denominator); // Burada sıfıra bölme yapılacak.

    match result {
        Ok(value) => println!("Sonuç: {}", value),
        Err(e) => {
            // Hata durumunda panikleme yerine hatayı yazdırıyoruz.
            println!("Hata oluştu: {}", e);
            // Panikleme ve programın sonlanmasını sağlıyoruz.
            panic!("Program kurtarılamaz bir durumda.");
        }
    }
}