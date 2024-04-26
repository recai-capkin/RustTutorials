# Rust Programlama Dili

Rust, performans, güvenlik ve eşzamanlılık konularında güçlü özellikler sunan modern, sistemler düzeyinde bir programlama dilidir.

Rust, yüksek performanslı ve güvenli yazılım geliştirme için tasarlanmış modern, sistem seviyesinde bir programlama dilidir. Mozilla tarafından başlatılan ve daha sonra geniş bir topluluk tarafından desteklenen bu dil, ilk kararlı sürümünü 2015 yılında yayınlamıştır. Rust, özellikle bellek güvenliği, eş zamanlılık ve sistem seviyesi detaylara erişim gibi alanlarda güçlü özellikler sunar. İşte Rust'ın çıkış sebepleri ve hangi ihtiyaçlara cevap verdiği hakkında bazı bilgiler:

### Neden Çıkmıştır?

1. **Bellek Güvenliği**: Rust, C ve C++ gibi dillerde sıkça karşılaşılan bellek yönetimi hatalarını ve güvenlik açıklarını önlemeyi amaçlar. Bellek güvenliği hataları, yazılım hatalarının ve güvenlik açıklarının en yaygın sebeplerindendir. Rust, bu tür hataları compile time'da (derleme sırasında) yakalayarak çalışma zamanı hatalarının önüne geçmeyi hedefler.

2. **Eş Zamanlılık**: Rust, eş zamanlı ve paralel programlama için güvenli desenler sunar. Bu sayede, çoklu iş parçacıklarının aynı anda çalıştırılması sırasında karşılaşılabilecek yarış koşulları ve veri yarışmaları gibi sorunları minimize eder.

3. **Performans**: Rust, sistem seviyesi performansı gerektiren uygulamalar için uygundur ve "zero-cost abstractions" (sıfır maliyetli soyutlamalar) ilkesine dayanır. Bu, Rust'ın soyutlamalarının çalışma zamanında ekstra bir maliyeti olmadığı anlamına gelir, yani C veya C++ ile yazılmış kod kadar hızlı çalışabilir.

### Hangi İhtiyaca Cevap Verir?

- **Sistem Programlama**: Rust, işletim sistemleri, dosya sistemleri, oyun motorları ve diğer sistem seviyesi bileşenler gibi düşük seviyeli sistem bileşenlerini geliştirmek için idealdir.
- **Gömülü Sistemler**: Gömülü cihazlar için gerekli olan yüksek performans ve düşük kaynak kullanımı gereksinimlerini karşılar.
- **Ağ Servisleri**: Yüksek eş zamanlılık ihtiyacı olan ve yüksek trafikli ağ servisleri geliştirirken güvenlik ve hızı ön planda tutar.
- **Kriptografi ve Güvenlik**: Güvenlik kritik uygulamalar için, bellek güvenliği sağlayarak ve güvenli kodlama desenlerini teşvik ederek ideal bir seçimdir.

Rust, bu alanlarda güvenlik ve performansı aynı anda sunarak, modern yazılım geliştirme pratiklerine uygun, güçlü bir dil olarak öne çıkar. Geliştiricilere, performansı korurken daha güvenli ve okunabilir kod yazma imkanı tanır.



## [Değişkenler ve Sabitler]

- **Değişkenler:** Rust'ta değişkenler varsayılan olarak sabittir (immutable). `mut` anahtar kelimesi ile değişkenler değiştirilebilir (mutable) hale getirilebilir.
- **Sabitler:** `const` anahtar kelimesi ile tanımlanır ve programın çalışma süresi boyunca değişmezler.

## [Veri Türleri](https://github.com/recai-capkin/RustTutorials/tree/main/Tutorial_5/variable)

- **Temel Veri Türleri:** i32, u32, f64, bool, char gibi.
- **Karmaşık Veri Türleri:** arrays, tuples, vectors, hash maps.

## [Kontrol Akışı]

- **if/else Yapıları:** Koşullu ifadeler.
- **Döngüler:** `loop`, `while`, `for` döngülerini içerir.
- **Pattern Matching:** `match` ifadesi ile desen eşleştirme.

## [Fonksiyonlar ve Makrolar]

- **Fonksiyonlar:** `fn` anahtar kelimesi ile tanımlanır.
- **Makrolar:** `macro_rules!` ile tanımlanır ve tekrar kullanılabilir kod parçacıkları oluşturur.

## [Hata Yönetimi]

- **Result ve Option Türleri:** Hataları yönetmek için `Result<T, E>` ve varlık/yokluk durumlarını ifade etmek için `Option<T>` kullanılır.
- **Panik Yönetimi:** Programın beklenmeyen durumlarda durmasını sağlar.

## [Sahiplik ve Borçlanma]

- **Ownership:** Bellek güvenliğini garanti altına alır.
- **Borrowing:** Değişkenlere referanslar üzerinden güvenli erişim.
- **Lifetime:** Referansların geçerlilik süresini kontrol eder.

## [Struct'lar ve Enum'lar](http://www.ornek.com)

- **Struct:** Karmaşık veri türlerini gruplamak için kullanılır.
- **Enum:** Birden fazla form veya türdeki değerleri tek bir tür altında toplar.

## [Modüller ve Paketler]

- **Modüller:** Kodu modüllere ayırarak düzenler (`mod` anahtar kelimesi ile).
- **Crate'ler:** Paket veya kütüphane olarak da bilinir; modüller ve yapılandırmaları içerir.
- **Cargo:** Rust'ın paket yöneticisi ve derleyicisidir.

## [Eşzamanlılık]

- **Thread'ler:** Rust veri yarışı olmadan eşzamanlı programlamayı destekler.
- **Async/Await:** Asenkron programlama için modern yaklaşım.

## [Makrolar]

- **Derive Makroları:** Bazı davranışları türetmek için otomatik kod üretir.
- **Procedural Makrolar:** Derleme zamanında kod üretir.

## [Trait'ler ve İmplementasyonlar]

- **Trait'ler:** Nesne yönelimli programlamadaki arayüzlere benzer, belirli fonksiyonaliteyi tanımlar.
- **Implementasyonlar:** Struct veya enum için fonksiyonları ve trait'leri tanımlar.

## [Gelişmiş Tip Sistemi]

- **Generics:** Parametre olarak tip alabilen yapılar ve fonksiyonlar, kod yeniden kullanımını artırır.
- **Type Inference:** Değişkenlerin tipleri çoğu zaman otomatik olarak belirlenebilir, böylece eksplisit tip belirtme gerekliliğini azaltır.
- **Traits as Bounds:** Generics kullanırken trait sınırlamaları ile tip güvenliği sağlanır.

## [Makro Sistemi]

- **Declarative Macros:** `macro_rules!` ile tanımlanır ve tekrarlanan kod yapısını azaltır.
- **Procedural Macros:** Derleme zamanında çalışan ve daha kompleks işlemler yapabilen makrolar. Özelleştirilmiş `#[derive]` attribute ve function makroları oluşturabilir.

## [Gelişmiş Eşzamanlılık Desteği]

- **Fearless Concurrency:** Rust bellek güvenliği garantileri sayesinde veri yarışı (race conditions) olmadan eşzamanlı kod yazmayı kolaylaştırır.
- **Message Passing:** Aktör modeline benzer şekilde thread'ler arası iletişim için mesaj geçişi kullanılabilir (`std::sync::mpsc`).
- **Shared State Concurrency:** Mutex ve diğer senkronizasyon mekanizmaları ile paylaşılan durum üzerinde thread'ler arası güvenli işlem yapılabilir.

## [Gelişmiş Hata Yönetimi]

- **Custom Error Types:** Rust `std::error::Error` trait'ini uygulayarak özel hata tipleri oluşturmanıza olanak tanır.
- **Error Handling with Result and Option:** Rust'ın `Result` ve `Option` tipleri hata yönetimini ve opsiyonel değerleri ele almayı tip güvenli hale getirir.

## [Düşük Seviye Kontrol]

- **No Garbage Collector:** Rust otomatik bellek yönetimi sağlar fakat bir çöp toplayıcıya (garbage collector) ihtiyaç duymaz. Bellek sahiplik ve ödünç verme kurallarıyla yönetilir.
- **Inline Assembly:** Rust, belirli durumlar için `asm!` makrosu ile inline assembly kodu yazmanıza olanak tanır.
- **Foreign Function Interface (FFI):** Rust, C gibi diğer dillerde yazılmış fonksiyonları çağırabilmenizi sağlar. Bu özellikle mevcut sistemlerle entegrasyon gerektiğinde faydalıdır.
