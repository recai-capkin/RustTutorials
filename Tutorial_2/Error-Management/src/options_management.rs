pub fn getAges(){
    let age =20;
    let users_ages = vec![Some(30), None, Some(age)];

    for age in users_ages {
        match age {
            Some(age) => println!("Kullanıcının yaşı: {}", age),
            None => println!("Yaş bilgisi mevcut değil"),
        }
    }
}


fn find_index(values: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in values.iter().enumerate() {
        if value == target {
            return Some(index); // Hedef değer bulunduğunda index'i döndür
        }
    }
    None // Hedef değer bulunamadıysa None döndür
}
pub fn get_Index(){
    let numbers = [1, 2, 3, 4, 5];
    match find_index(&numbers, 3) {
        Some(i) => println!("Değer dizinin {} indexinde bulundu.", i),
        None => println!("Değer bulunamadı."),
    }
}