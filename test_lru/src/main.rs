use lru::LruCache;

fn main() {
    let mut cache = LruCache::new(2);
    cache.put("apple", 3);
    cache.put("banana", 2);
    println!("{:?}", cache);

    println!("apple:  {:?}", cache.get(&"apple"));
    println!("banana: {:?}", cache.get(&"banana"));
    println!("banana: {:?}", cache.get(&"orange"));
    println!("");

    cache.get(&"apple");
    cache.put("orange", 4); // cause eviction of banana since it is LRU
    println!("apple:  {:?}", cache.get(&"apple"));
    println!("banana: {:?}", cache.get(&"banana"));
    println!("orange: {:?}", cache.get(&"orange"));
    println!("");

    cache.put("banana", 5); // cause eviction of apple since orange was added after
    println!("apple:  {:?}", cache.get(&"apple"));
    println!("banana: {:?}", cache.get(&"banana"));
    println!("orange: {:?}", cache.get(&"orange"));
    println!("");
}
