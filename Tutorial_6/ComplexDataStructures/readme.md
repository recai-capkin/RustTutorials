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
