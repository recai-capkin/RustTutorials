# Data Structures

Rust dilinde kapsamlı olarak veri yapılarını inceliyoruz.

## Rust'da :: operatörü ne işe yarar?

Rust'da `::` operatörü, "kapsam çözünürlüğü operatörü" olarak kullanılır ve belirli bir tür veya modülün ilişkili fonksiyonlarını veya sabitlerini çağırmak için kullanılır. Bu operatör, bir tip (veya modül) adından sonra gelir ve ona bağlı bir fonksiyonun veya sabitin adını belirtir.

Örneğinizde, `String::from("Alice")` ifadesi `from` adında bir fonksiyonu kullanıyor. Bu fonksiyon, `String` türüne ait bir "ilişkili fonksiyon"dur (bazen "statik metod" olarak da adlandırılır). `String::from` fonksiyonu, verilen bir dize dilimini (`&str`) alır ve onu bir `String` nesnesine dönüştürür. Bu dönüştürme işlemi, `String` türü için ayrılmış olan heap belleğinde yeni bir `String` nesnesi oluşturarak gerçekleştirilir.

`String::from` fonksiyonunun kullanımı, Rust'ta yaygın olarak görülen bir kalıptır. Rust, bellek güvenliği garantileri sağlayan bir dil olduğu için, `String` tipindeki verilerin yönetimi (örneğin belleğe alınması, boyutunun değiştirilmesi gibi işlemler) `String` türü tarafından kapsülleme ile sağlanır. `String::from` fonksiyonu, bu tür işlemleri kolaylaştırmak için tasarlanmıştır ve bir dize dilimini (`&str`), sahiplik haklarına sahip (owned) bir `String` nesnesine dönüştürmek için kullanılır. Bu, Rust'ın bellek yönetim özelliklerinden tam olarak yararlanmanızı sağlar.

## Rust'da neden snake case kullanılır?

**Tutarlılık:** Rust'ın stil rehberi, kodun kolay okunabilir ve tutarlı olmasını sağlamak için var. Snake case kullanımı, Rust ekosistemine katkıda bulunan birçok geliştirici tarafından benimsenmiş ve uygulanmıştır. Bu tutarlılık, Rust'ın farklı kütüphaneleri ve projeleri arasında geçiş yapmayı ve anlamayı kolaylaştırır.

**Okunabilirlik:** Snake case, kelime sınırlarını açıkça belirler. Kelimeler arasındaki alt çizgiler, isimlerin okunmasını ve anlaşılmasını kolaylaştırır, özellikle de uzun tanımlayıcılar için. Bu, tanımlayıcıların görsel olarak daha hızlı işlenmesine yardımcı olur.

**Dilin Tasarım Felsefesi:** Rust, güvenlik, hız ve eş zamanlılık konularında yüksek performans sağlamaya odaklanmış bir dildir. Bu odak, dilin tasarım ve kullanımındaki diğer yönleri de etkiler. Rust, programcıların dikkatini dilin bu temel özelliklerine odaklamasına yardımcı olacak şekilde tasarlanmış bir stil rehberine sahiptir. Bu stil rehberi, programcıların kod üzerinde daha az süre harcamalarını ve daha etkili kod yazmalarını sağlar.

**Araç Desteği:** Rust, `rustfmt` gibi araçlarla birlikte gelir. `rustfmt`, Rust kodunu otomatik olarak biçimlendirir ve stil kılavuzuna uygun hale getirir. `rustfmt` kullanarak, geliştiriciler snake case ve diğer stil kurallarını otomatik olarak uygulayabilirler, bu da manuel stil düzeltmelerine harcanacak zamanı azaltır. Bu nedenlerle, Rust dilinde snake case kullanımı, dilin tasarımının ve topluluğunun önemli bir parçası haline gelmiştir. Dilin bütünüyle uyumlu bir şekilde işlemesini ve geliştiriciler arasında anlaşmazlıkları minimize etmeyi amaçlar.


## Vec<T> Yapısı ve Özellikleri?

Rust programlama dilinde `Vec<T>` yapısı, boyutu çalışma zamanında değiştirilebilen ve aynı türden (`T` türünde) elemanları içeren bir koleksiyon türüdür. Rust'ın standart kütüphanesinde dinamik dizi veya esnek dizi olarak işlev görür ve Rust'ta yaygın olarak kullanılan bir veri yapısıdır. Ayrıca, dilin temel güvenlik ve performans özelliklerini de destekler.

**RAM'de Depolanma Yapısı:**

`Vec<T>`'nin elemanları heap üzerinde saklanır. Bu, çalışma zamanı sırasında veri boyutunun ve kapasitesinin değişebilmesi için gereklidir. `Vec<T>` genellikle üç ana bileşenden oluşur:

- **Pointer (İşaretçi):** Heap üzerindeki eleman dizisine bir işaretçi.
- **Length (Uzunluk):** Vektörde şu anda kaç eleman olduğunu gösterir.
- **Capacity (Kapasite):** Vektörün bellekte kapladığı maksimum alanı (eleman sayısı olarak) gösterir. Bu değer, vektör yeniden boyutlandırılmadan önce kaç elemanın saklanabileceğini belirler.

Bu yapı, `Vec<T>`'ye eleman eklenirken veya çıkarılırken bellek yönetiminin etkili bir şekilde yapılmasını sağlar. Eğer vektörün uzunluğu kapasitesini aşarsa, daha büyük bir hafıza bloğu tahsis edilir ve mevcut elemanlar yeni bloğa kopyalanır.


**Type Safety Özellikleri:**

Rust'ın `Vec<T>` kullanımındaki tip güvenliği (type safety), dilin genel tasarım felsefesiyle uyumludur. `Vec<T>` ile ilgili tip güvenliği özellikleri şunlardır:

- **Homogeneous Collection:** `Vec<T>` yalnızca tek bir veri türünden (`T`) elemanları saklar. Bu, koleksiyonun tüm elemanlarının aynı türde olmasını garantiler, böylece türle ilgili hataların önüne geçilir.
- **Compile-Time Type Checking:** Rust derleyicisi, `Vec<T>` içine eklenen elemanların türünü derleme zamanında kontrol eder. Yanlış türde bir değer eklenmeye çalışıldığında, derleyici hata verir. Bu, çalışma zamanı hatalarının önlenmesine yardımcı olur.
- **Memory Safety:** `Vec<T>` kullanımı, bellek güvenliği açısından da korumalıdır. Rust'ın sahiplik (ownership) ve ödünç verme (borrowing) kuralları, `Vec<T>` üzerinde işlem yaparken bellek güvenliğinin korunmasını sağlar. Örneğin, bir vektör üzerinde geçersiz bir indekse erişmeye çalışmak, derleme zamanı veya çalışma zamanı hatalarıyla sonuçlanır.
- **Bounds Checking:** Rust, `Vec<T>`'ye erişimde sınır denetimi yapar. İndeks operatörü (`vec[index]`) kullanıldığında, Rust çalışma zamanında indeksin geçerli olup olmadığını kontrol eder. Eğer indeks dizi sınırları dışındaysa, bir hata (panic) meydana gelir, böylece bellek güvenliği ihlallerinin önüne geçilir.

# Girdi İşleme ve Hata Yönetimi

Bu belgede, Rust programlama dilinde kullanıcı girdisinin nasıl işlendiği ve olası hata durumlarının nasıl ele alındığı açıklanmaktadır.

## `.parse::<u32>()` Metodu ve `unwrap_or` Kullanımı

Kullanıcıdan alınan girdiyi sayısal bir değere (`u32`) dönüştürmek için `.parse::<u32>()` metodu kullanılır. Bu metod, başarılı bir dönüşüm gerçekleştiğinde `Ok(u32)` şeklinde bir `Result` tipi döndürür. Başarısız olması durumunda ise `Err` ile bir hata döner.

### Kullanımı

```rust
let count = read_input("Kaç kullanıcı ekleyeceksiniz? ").parse::<u32>().unwrap_or(0);
