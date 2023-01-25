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