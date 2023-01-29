## 无畏并发

### Thread

使用thread::spawn创建子线程，运行里面指定的代码

返回值为JoinHandle，使用join方法，在当前线程阻塞运行直到handle对应的线程运行结束

```rust
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

```

#### Move闭包

Move闭包可以把主线程的变量的所有权转移给spawn的子线程，让子线程处理数据

在闭包的参数 ||之前加上move



### 消息传递

#### 创建 Channel

mpsc: multiple producer single consumer

```rust
let (tx, rx) = mpsc::channel();

// move the tx to the thread
thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```

- tx发送，此时如果对应的rx已经无效则panic
- 主线程接收
- send会将所有权转移给recv