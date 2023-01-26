## Package Module Crate

### Package&Crate

#### Crate

Crate的类型是binary或者library

crate root是源代码文件，编译器从此处开始构建根Module

#### Package

包含Cargo.toml，描述如何构建Crates

包含0-1个library，任意个binary，至少有一个crate

#### 惯例

- 如果src/main.rs存在，则有一个binary的crate，名称与package名相同
- 如果src/lib.rs存在，则有一个library的crate，名称与package相同
- 多个binary放在src/bin下，每个文件是单独的binary



### Module

在一个crate中将代码分组

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

一个文件自动对应一个名叫crate的module



### 路径Path

使用绝对路径或者找到某个条目

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}
```



#### 私有边界

模块中的代码默认是私有的，如函数，struct等

父模块不能访问子模块中私有内容，子模块能够访问父模块中所有内容

同级可以相互访问pub内容



#### super

访问上一层mod的内容



#### pub

##### struct

struct加上关键字pub之后，struct本身是公共的，但是内部的字段和方法不是公共的，需要单独添加

##### enum

pub enum使得enum和其中的枚举值都是公共的



#### use

使用use path引用一个mod，use ...::mod之后，在当前路径下可以通过mod::访问mod的内容

use引入的惯例：

- 引入函数时，引入到它的父模块级别，以区分不同的函数定义
- 对于struct，一般直接引入struct本身，如果存在冲突则引入父级
  - 还可以使用as，引用为不同的名称



#### pub use

使用use引入之后在当前路径下可见，但是是当前路径私有的，其他外部路径包含当前路径时不能访问，加上pub之后可以把use的内容暴露给外部



### 使用外部包

- 在Cargo.toml中加入依赖的名字
- 使用use引入

std是默认的依赖，不需要包含，但是依然需要use



#### 使用嵌套路径减少引入代码

```rust
use std::{cmp::Ordering, io};
```

::self可以引入自己

*通配符会引入所有公共内容



### 将模块内容移动到其他文件

如果一个文件中不存在mod的定义，只有名字，则会自动在当前目录下寻找模块名对应的文件，其中有mod定义

```rust
mod hosting;
```

嵌套的mod，父级是一个文件夹