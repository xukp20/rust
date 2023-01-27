## 测试

使用test标注测试函数

```rust
#[test]
```

定义函数的测试属性

使用cargo test运行所有测试，运行所有标注test的函数

使用cargo创建library时，自动携带test module

创建库项目

```shell
 cargo new adder --lib
```

lib.rs

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

#### 测试失败

测试函数出现panic，则测试失败

```rust
#[test]
fn fail() {
    panic!("fail testcase!");
}
```



### 断言 Assert

#### assert!

检查bool值是否为true，false则panic

#### assert_eq! assert_ne!

失败时调用Debug打印两个参数

#### 自定义信息

assert，assert_eq，assert_ne可以增加一个参数为信息



### 验证错误处理

检查是否发生了panic，使用should_panic属性定义测试函数应该panic

should_panic属性后面可以加expected=panic信息

```rust
#[test]
#[should_panic(expected="info")]
```

panic信息包括info时通过测试



### 使用Result

测例可以返回Result，若为Ok则通过，Err则失败



### 单元测试

#[cfg(test)]标注单元测试，和测试的源代码在同一文件内，只有cargo test时才编译



### 集成测试

在tests目录下创建集成测试文件，每个文件对应一个crate