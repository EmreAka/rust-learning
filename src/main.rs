#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({}) - {}",
            self.headline, self.author, self.location, self.content
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("Default imp");
    }
}

// Trait bound

/* pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
} */

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Blanket implementations
impl<T: Summary> TestTrait for T {}

pub trait TestTrait {
    fn test_le_beni(&self) {
        println!("Testledim seni! 🥰🥰🥰 🤢🤢");
    }
}

/* fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
} */

fn main() {
    let article = NewsArticle {
        author: String::from("Emre"),
        content: String::from("Rust language service is immitating Angular language service"),
        headline: String::from("Şok şok şok!"),
        location: String::from("Istanbul/Turkey"),
    };

    let summarize = article.summarize();

    println!("summarize: {}", summarize);

    notify(&article);

    println!("{:?}", article);

    article.test_le_beni()

}
