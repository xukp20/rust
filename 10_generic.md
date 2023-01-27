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

```rust
let r;
{
    let x = 5;
    r = &x;
}
println!("{}", r);
```

因为x在代码块之后无效，x的引用失效，因此r的输出有错

如果函数的返回值与参数有关，在函数的返回类型中，定义生命周期，使用'a

### 生命周期标注

' 开头，&i32 -> & 'a i32

一般描述返回值和参数的生命周期关系

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    ...
}
```

声明x, y, 返回值的生命周期不小于a

a对应的会是x,y中比较短的生命周期

- 返回值是引用时，肯定来自于参数，则将相关的参数添加生命周期参数，返回值的生命周期将为最短的那个
- 如果返回值是引用，但是不来自于参数，那么肯定是在函数内创建的，离开函数之后就失效了，一定出错
- 如果返回值不是引用，那么不需指定生命周期，在函数内创建返回值

#### struct的生命周期

struct的生命周期是内部所有字段的生命周期中的最小值

#### 生命周期省略规则

输入生命周期：参数的生命周期

输出生命周期：返回值的生命周期

三条规则：

- 每个引用类型的参数有自己的生命周期
- 若只有一个输入，则输出= 输入
- 如果多个输入，但是一个是&self,&mut self，即为方法，则输出为self的生命周期

使用三条规则不能判断生命周期则报错

### 静态生命周期

’static 是整个程序的持续时间

所有字符串字面值为‘static的

