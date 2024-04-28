# Unsafe Kullanımı Açıklaması

Rust dilinde, genellikle güvenlik garantileri sağlamak amacıyla dilin çoğu özelliği güvenli bir şekilde tasarlanmıştır. Ancak, bazı düşük seviyeli işlemler, dilin bu güvenli yapısının dışına çıkmayı gerektirir. İşte burada `unsafe` anahtar kelimesi devreye girer. Bu belge, `unsafe` kullanımının örnek bir kod parçası üzerinden nasıl işlediğini açıklamaktadır.

## Kod Açıklaması

```rust
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
```
## Ham İşaretçiler

Ham işaretçiler (`*const T` ve `*mut T`), Rust'ın mülkiyet ve ödünç verme kurallarını ihlal edebilir. Örneğin, bu kodda `num` değişkenine hem sabit (`*const i32`) hem de değiştirilebilir (`*mut i32`) ham işaretçiler atanmıştır. Bu işaretçiler üzerinden yapılan değişiklikler, `unsafe` bloğu içerisinde gerçekleştirilmelidir çünkü derleyici, bu işaretçilerin güvenli bir şekilde kullanıldığını garantileyemez.

## Dış Fonksiyonlar

`extern "C"` bloğu, Rust dışındaki fonksiyonları kullanabilmek için tanımlanmıştır. Bu örnekte, C dilinin standart kütüphanesinden `abs` fonksiyonu kullanılmıştır. Bu fonksiyonlar genellikle güvenli olmayan kabul edilir çünkü Rust, bu fonksiyonların iç yapısını ve uygulamasını kontrol edemez.

## Sonuç

`unsafe` bloğu, Rust programcısına daha fazla kontrol ve esneklik sağlar; ancak, bu kullanım ekstra dikkat ve dikkatli bir yönetim gerektirir. `unsafe` kullanımı, yalnızca gerçekten gerekli olduğunda ve alternatif güvenli yaklaşımların yetersiz kaldığı durumlarda tercih edilmelidir.
