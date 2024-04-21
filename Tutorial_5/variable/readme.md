<h1>Stack Hafızada Saklama</h1>
Rust'da, değişkenler ve yapılar genellikle "stack" adı verilen bölgede saklanır.
Stack, veri yapıları için hızlı erişim sağlayan ve bellek tahsisi ile geri bırakma işlemlerinin oldukça verimli olduğu bir hafıza alanıdır.
Gösterdiğiniz MyStruct örneğinde:

let my_struct_instance = MyStruct { field1: 10 };

Bu satırda my_struct_instance isimli bir yapı örneği oluşturuluyor. Bu yapı örneği, fonksiyonun yerel değişkeni olarak stack'te tutulur. Stack üzerinde tutulmasının sebebi, bu örneğin fonksiyon çağrısıyla oluşturulması ve fonksiyonun sonlanmasıyla birlikte kapsam dışında kalarak bellekten otomatik olarak temizlenmesidir. Bu süreç Rust'ın "ownership" modeli ile yönetilir, böylece manuel bellek yönetimi gerekmez.

<h1>Heap Hafızada Saklama</h1>
Bazı durumlarda, yapılar "heap" adı verilen bellek bölgesinde saklanabilir. Heap, daha büyük veri yapıları veya ömürleri fonksiyonlar veya belirli kapsamlarla sınırlı olmayan yapılar için kullanılır. Eğer bir yapıyı heap üzerinde saklamak istiyorsanız, Rust'ta genellikle Box, Rc, veya Arc gibi pointer türlerinden yararlanılır. 
Örnek:

let my_struct_instance = Box::new(MyStruct { field1: 10 });

<h1>Sonuç</h1>
Yapınızın use_struct fonksiyonundaki kullanımı göz önüne alındığında, MyStruct yapı örneği stack üzerinde saklanır, çünkü doğrudan yerel bir değişken olarak tanımlanmış ve bir pointer aracılığıyla heap'e atılmamıştır. Rust'ın bellek yönetimi modeli, bu tür kullanımlar için otomatik olarak gereken bellek yönetimini sağlar ve performans açısından oldukça etkilidir.