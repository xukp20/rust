## 智能指针

引用只借用数据，智能指针一般拥有所有权

String和Vec<T>都是智能指针

### 实现

通常使用struct实现，实现了Deref和Drop的Trait



### Box< T >

在Heap上存储数据，Stack上有对应的指针指向数据

实现了Deref和Drop，

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

Box会将5的数据放在Heap上，离开作用域时，Box指针会被析构，且数据被释放

#### Box实现递归类型

链表的实现中，链表的递归定义中，后面需要用Box声明为指针类型



### Deref

使得struct能够类似引用来解引用（使用*）

定义了deref方法，返回的是内部的值的引用，因此可以用来解引用

即 y 相当于 y.deref()

#### 隐式解引用

实现了Deref的对象的引用可以自动转化为deref返回的引用类型，可以用于函数传参时的自动转换

```rust
fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let m = Box::new(String::from("Rust"));

    hello(&m);

    hello("Rust");
}
```

- Box实现了deref，所以m的引用会解为&String
- String本身也实现了deref，解为&str传入

##### 解引用与可变性

DerefMut Trait对应可变引用的*

若T实现了DerefMut，则&mut T可以转为&a 或者 &mut a



### Drop

类似析构函数，任何类型都可以实现，在值离开作用域时调用，只需实现一个drop方法

```rust
struct Dropper {
    data: String,
}

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("Dropping dropper with {}", self.data);
    }
}
```

- 不能显式地调用drop方法
- 可以使用std::mem::drop函数，传入实例对象，提前析构对象



### Rc < T > 引用计数智能指针

reference counting，通过维护引用个数计数器实现

使用场景：在Heap上分配单个数据，被程序的多个部分读取（只读），无法确定谁最后使用

只能用于单线程

例：b,c两个list共用相同的尾部a

```rust
use std::rc::Rc;

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Nil)));
    println!("{}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));

    let _c = Cons(4, Rc::clone(&a));

    println!("{}", Rc::strong_count(&a));
}
```

- List a 本身是一个Rc指向的，new的时候引用计数为1
- Rc::clone 增加引用计数，返回不可变引用



### RefCell和内部可变性

RefCell在运行时才检查借用的可变性