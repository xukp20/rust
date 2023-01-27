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


// lifespan

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>
    (x: &'a str, y: &'a str, ann: T) -> &'a str
where T:Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let news = News {
        title: String::from("News"),
        content: String::from("Hello world!"),
    };

    notify1(&news);
    notify(news);
}
