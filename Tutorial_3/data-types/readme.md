# Veri Depolama Yapıları

Bu belge, Rust programlama dilinde verilerin nasıl depolandığını açıklamaktadır. İki ana depolama alanı vardır: Stack ve Heap.

## Stack'te Depolanan Veri Tipleri

Stack, sabit boyutlu veri depolamak için idealdir. Aşağıda stack'te tipik olarak depolanan veri tipleri listelenmiştir:

- **Temel Veri Tipleri**: `i32`, `u64`, `f32`, `f64`, `char`, `bool` gibi.
- **Tüpler (Tuples)**: Sabit boyutlu ve farklı tiplerdeki verileri bir arada tutabilir. Örnek: `(i32, f64, char)`.
- **Diziler (Arrays)**: Sabit boyutlu ve aynı tipteki verileri bir arada tutar. Örnek: `[i32; 5]` (5 elemanlı bir `i32` dizisi).
- **Struct'lar (Yapılar)**: Eğer bir struct sadece stack'te depolanan tipleri içeriyorsa, o struct da stack'te depolanır. Örnek: `struct Point { x: i32, y: i32 }`.
- **Enum'lar (Sabit Boyutlu)**: Sabit boyutlu veri içeren enumlar. Örnek: `Option<i32>` veya `Result<T, E>` (T ve E sabit boyutlu ise).

## Heap'te Depolanan Veri Tipleri

Heap, çalışma zamanında boyutları belli olmayan veya boyutu değişebilen veriler için kullanılır. Aşağıda heap'te tipik olarak depolanan veri tipleri listelenmiştir:

- **String**: Rust'taki `String` tipi, çalışma zamanında boyutu değişebilen metin verisini depolar.
- **Vektörler (Vec<T>)**: Dinamik boyutlu dizi. Örnek: `Vec<i32>` bellekte dinamik olarak büyüyebilir veya küçülebilir.
- **Box<T>**: `Box` bir pointer türüdür ve heap'te bir veriyi saklar. Özellikle, tek bir değeri heap'te depolamak için kullanılır.
- **Rc<T>** ve **Arc<T>**: Referans sayılı akıllı pointer'lar. `Rc` tek iş parçacıklı ortamlar için, `Arc` ise çok iş parçacıklı ortamlar için tasarlanmıştır. Her ikisi de heap'te depolanan verilere sahip olur.

## Diğer Karmaşık Tipler

- **Struct'lar ve Enum'lar (Heap İçeren)**: Eğer bir struct veya enum, `String`, `Vec<T>`, `Box<T>`, `Rc<T>`, `Arc<T>` gibi heap bellek kullanımını gerektiren tipler içeriyorsa, bu struct veya enum kendi içinde hem stack hem de heap kullanımına sahip olur. Örnek: struct içinde bir `Vec<T>` varsa, struct'ın kendisi stack'te depolanırken, `Vec<T>`'nin verileri heap'te depolanır.
