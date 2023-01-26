## 泛型

### 函数定义中的泛型

参数和返回类型是泛型，函数名后面加<T>



### 结构体泛型

```rust
struct Point<T> {
    ...
}
```

实现的方法impl需要写为 impl<T>，其中定义的是对应模板泛型的方法，还可以使用impl<i32>定义单独针对i32的方法

### 枚举泛型

一般让变体持有的数据为泛型，如Option<T>的Some(T)





## Trait

描述类型的功能，能够定义共享的行为，泛型函数可以限制泛型所需要具有的Trait

### 定义

一个Trait的定义中有若干方法的签名，没有实现，对应于实现所需功能的一组行为；

> 每个方法类似interface，Trait类似虚类

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### 实现

在特定类型上实现Trait

```rust
impl Trait_name for struct_name {
    ...
}
```

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct News {
    pub title: String,
    pub content: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{}, {}", self.title, self.content)
    }
}

fn main() {
    let news = News {
        title: String::from("News"),
        content: String::from("Hello world!"),
    };

    println!("{}", news.summarize());
}
```

实现约束：

- struct类型和所要实现的trait至少一个在本地即当前的crate中定义，而不能给一个外部struct实现外部trait



### Trait作为参数

函数的参数约束为实现了某个特定Trait的类型

- 写法一：参数类型
  ```rust
  fn notify(item: impl Summary) {
      println!("Breaking! {}", item.summarize());
  }
  ```

- 写法二：Trait bound，类似泛型的方式

  ```rust
  fn notify1<T: Summary>(item: T){
      println!("Breaking! {}", item.summarize());
  }
  ```

指定多个Trait在Trait之间加 +

还可以把Trait bound中的限制放在方法签名后面

```rust
fn notify<T>(item: T) where T: Summary{
    
}
```

#### 返回类型

-> impl Summary限制返回类型需要实现Summary



### 为特定Trait实现impl

可以像泛型一样在impl上限制实现的Trait类型

```rust
impl<T: A> B for T {} 
```

为所有实现了A的T类型实现B



## 生命周期

引用保持有效的作用域范围