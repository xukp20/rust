### 不可恢复的错误&Panic

默认情况下，panic发生时会沿着调用栈返回，清理所有数据；

还可以使用中止，abort，遇到panic时立即终止执行，操作系统来清理内存



### 可恢复的错误&Result

比如打开文件时，文件不存在

Result:

```rust
enum Result<T, E>{
    Ok(T),
    Err(E),
}
```

T为数据类型，E为错误类型

#### 匹配不同的错误

```rust
let f = File::open("Hello.txt");

let _f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("Hello.txt") {
            Ok(fc) => fc,
            Err(_e) => panic!("Error creating file"),
        },
        _other_error => panic!("Error opening file"),
    }
};
```

#### unwarp

match的快捷方法

在Ok时返回Ok的数据，否则调用panic

```rust
let f = File::open("Hello.txt").unwrap()
```

#### expect

与unwrap类似，可以提供错误信息作为参数



### 传播错误

在出错的函数中不处理，传播给调用者

只需要将函数的返回类型改为Result<T,E>即可，在成功的时候就是Ok(T)，否则返回Err即可

- 可以使用？运算符，Result为Ok时等于表达式结果，如果为Err则执行函数return Err
  - ？运算符会自动处理Err类型为函数的返回类型



### 何时使用Panic



### 使用自定义类型进行验证

