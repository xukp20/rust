### 定义&实例化

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

最后一个后面也有，

实例化时将定义的字段后面的类型改为数据，实例化struct时，必须要赋值所有字段

可以单独修改struct的字段值，struct本身需要是mnt的



### 更新

使用旧的struct实例创建新实例，将部分字段赋为原实例的值，部分修改为新的值，可以省略相同的值

```rust
let user2 = User{
    email: String::from("123"),
    ..user1
}
```

除了email以外与user1相同



### Tuple struct

类似Tuple的struct，整体有名称，内部字段没有名称，相当于具名tuple

```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

可以使用tuple的访问方式



### Unit-Like Struct 

没有任何字段的struct，与单元()类似

用于实现Trait，而不包括数据



### 所有权

struct拥有所有字段的数据

如果struct的字段是引用，则需要使用生命周期



### 方法

方法的第一个参数是self

方法通过impl进行定义

```rust

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.length * self.width
    }
}

fn main() {
    let rect = &Rectangle { width: 20, length: 50 };
    println!("{}", rect.area());

    println!("{:#?}", rect);
}
```

方法的第一个参数可以是self,&self, &mnt self

rust会自动解引用，无论self的参数类型是什么，调用时都是rect.func()



### 关联函数

在impl块里定义的函数，如果第一个参数不是self，则为关联函数，（相当于静态方法）

可以有多个impl块

```rust
impl Rectangle {
    fn area(&self) -> u32{
        self.length * self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}
```

