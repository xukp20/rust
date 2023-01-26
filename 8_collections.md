## 常用集合

### Vector

```rust
Vec<T>
```

```rust
fn main() {
    // new vector
    let mut v: Vec<i32> = Vec::new();

    // use vec!
    let v1 = vec![1, 2, 3];

    // add item
    v.push(1);
}
```

离开作用域之后自动清理

#### 读取

-  索引
- get，返回Option枚举

```rust
// index
let third = &v1[2];
println!("{}", third);

// get
match v.get(2) {
    Some(third) => println!("{}", third),
    None => println!("Missing element"),
}
```

#### 遍历

使用不可变引用

```rust
// go through
for i in &v1 {
    println!("{}", i);
}
```





### String

str是语言核心类型，String是标准库中定义的

#### 创建

- new
- from
- to_string：可用于实现了Display的对象

```rust
// create
let mut s = "Hello".to_string();
s = String::from("Hello World");
```

#### 更新

- push_str：添加字符串
- push：添加字符
- +：String + &str，前者会失去所有权，可以加&String，因为有自动转换

```rust
// update
s.push_str("111");
s.push('l');            
let s1 = String::from("HaHa");
let s2 = s + &s1;

println!("{}", s2);
```

- format：能够使用与println相似的方式构造String，不会获得所有权

#### 索引

String不支持索引表示，因为每个字符占据的字节数可能不一样，字节索引可能无意义

- bytes：返回每一个字节
- chars：返回unicode标量值

#### 切片

& + [a..b]创建切片，根据字节分割

如果切割的结果不是在字符的边界，在运行时出错





### HashMap

HashMap<K,V>

不在预导入模块中，需要导入

```rust
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 5);
```

所有权会转移给HashMap，如果有Copy，则复制，否则Move

如果插入的是引用类型，则不会移动所有权，但是在HashMap使用的时间内，被引用的值必须有效

#### get

get方法通过Key的引用，返回Option<V>

#### for 

```rust
for (k, v) in &map {...}
```

#### 更新

entry判断是否存在，or_insert接下来在若不存在时插入

entry返回VacantEntry

or_insert如果发现前面entry存在，则返回对应V的可变引用；否则插入新的Entry，之后返回新的V的可变引用

```rust
scores.entry(String::from("Yellow")).or_insert(10);
```

