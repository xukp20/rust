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

fn notify(item: impl Summary) {
    println!("Breaking! {}", item.summarize());
}

fn notify1<T: Summary>(item: &T){
    println!("Breaking! {}", item.summarize());
}


fn main() {
    let news = News {
        title: String::from("News"),
        content: String::from("Hello world!"),
    };

    notify1(&news);
    notify(news);
}
