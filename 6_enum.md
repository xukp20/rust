## 枚举与类型匹配

### 枚举的变体

可以在枚举的值的后面附加变体数据，是tuple类型

```rust
enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}
```



### Option枚举

定义于标准库中，Rust中没有NULL，Option<T> 描述可能不存在的值

```rust
enum Option<T> {
    Some(T),
    None,
}

Option<T> 
Some(T)	// 存在时类型，能够直接转为T类型
None	// 不存在
```

一个变量被声明为Option之后，说明它可能为空；在将其作为T使用之前，必须手动将其转为T类型



### match

控制流运算符，模式匹配

模式 => 代码，匹配成功的模式执行后面的代码，

可以将enum对应的值绑定到后面的代码块中，类似参数传递

```rust
enum Coin{
    Penny, 
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State{
    A,
    B,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quarter in {:#?}", state);
            25
        },
    }
}
```

可以使用match来匹配Option<T>，对于None和Some<T>进行不同操作

match必须处理所有可能性，或者使用下划线_匹配其他所有情况；





### if let

只关心match中的一种情况，可以有else

```rust
match v {
    Some(3) => ...,
    _ => (),
}

//等价于
if let Some(3) = v {
    ...
}
```

