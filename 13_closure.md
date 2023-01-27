## 闭包

可以捕获其所在环境的匿名函数

闭包被定义为变量，可以在之后调用

### 定义

```rust
let A_closure = |num| {
    ....
};
```

- 参数在|之间
- 函数体与一般的函数相同
- 闭包可以不标注参数和返回值类型，因为一般只在上下文使用，可以推断
  - 但是一个闭包只会被推断出一种参数类型和返回值类型，即同一个闭包在上下文不能使用不同种类的参数进行调用

### struct持有闭包

struct中的字段必须有明确的类型，而每个闭包都是不同的匿名类型，所以应该使用泛型来匹配闭包字段，闭包实现Fn Trait

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher<T> 
where 
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation: calculation, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

- value接口会判断value值是否存在，如果不存在就使用闭包计算value
- Cacher的效果是返回的value一定对应的是第一次的参数，如果之后的参数不一样了，value依然不变，直接返回
- T指定了闭包类型



### 使用闭包捕获环境

闭包可以使用定义闭包的作用域中的值

```rust
let x = 4;
let eq_closure = |z| {z == x}
```

函数不允许从外面环境捕获变量

会产生内存开销

#### 获取值的方式

- FnOnce：取得所有权，只能调用一次，因为环境中的值会失去所有权
- FnMut：可变借用
- Fn：不可变借用

根据闭包内部的使用推断使用哪个Trait，所有的都实现了FnOnce，没有移动变量的实现了FnMut，进一步没有改变捕获变量的值则实现了Fn

##### move 关键字

闭包定义之前加上move关键字，强制获得参数的所有权