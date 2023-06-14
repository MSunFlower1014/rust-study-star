## Vector  
创建集合：  
```rust 
let mut v = Vec::new();  
```
添加元素  
```rust 
v.push();  
```
获取:  
```rust 
let does_not_exist = &v[100];//索引越界时异常  
let does_not_exist = v.get(100);//索引越界时返回None Option  
```
## String
String 是一个 Vec<u8> 的封装  
字符遍历： 
```rust 
for c in "Зд".chars() {  
    println!("{c}");  
}  
```
字节遍历  
```rust 
for b in "Зд".bytes() {  
    println!("{b}");  
}  
```
## HashMap
数据插入：
```rust 
let mut scores = HashMap::new();  
scores.insert(String::from("Blue"), 10);  
```

根据旧值更新：
```rust
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

```