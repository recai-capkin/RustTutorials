<!DOCTYPE html>
<html lang="tr">
<head>
    
</head>
<body>
    <div class="container">
        <header>
            <h1>Data Structures</h1>
            <p>Rust dilinde kapsamlı olarak veri yapılarını inceliyoruz</p>
        </header>
        <section>
            <h2>Rust'da :: operatörü ne işe yarar?</h2>
            <p>Rust'da :: operatörü, "kapsam çözünürlüğü operatörü" olarak kullanılır ve belirli bir tür veya modülün ilişkili fonksiyonlarını veya sabitlerini çağırmak için kullanılır. Bu operatör, bir tip (veya modül) adından sonra gelir ve ona bağlı bir fonksiyonun veya sabitin adını belirtir.

Örneğinizde, String::from("Alice") ifadesi from adında bir fonksiyonu kullanıyor. Bu fonksiyon, String türüne ait bir "ilişkili fonksiyon"dur (bazen "statik metod" olarak da adlandırılır). String::from fonksiyonu, verilen bir dize dilimini (&str) alır ve onu bir String nesnesine dönüştürür. Bu dönüştürme işlemi, String türü için ayrılmış olan heap belleğinde yeni bir String nesnesi oluşturarak gerçekleştirilir.

String::from fonksiyonunun kullanımı, Rust'ta yaygın olarak görülen bir kalıptır. Rust, bellek güvenliği garantileri sağlayan bir dil olduğu için, String tipindeki verilerin yönetimi (örneğin belleğe alınması, boyutunun değiştirilmesi gibi işlemler) String türü tarafından kapsülleme ile sağlanır. String::from fonksiyonu, bu tür işlemleri kolaylaştırmak için tasarlanmıştır ve bir dize dilimini (&str), sahiplik haklarına sahip (owned) bir String nesnesine dönüştürmek için kullanılır. Bu, Rust'ın bellek yönetim özelliklerinden tam olarak yararlanmanızı sağlar.</p>
        </section>
        
    </div>
</body>
</html>
