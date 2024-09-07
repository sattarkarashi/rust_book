fn main() {
    // Traits are like interfaces in other programming languages, they are an effort to show the shared behaviors

    // Define a trait:

    pub trait Summary {
        fn summarize(&self) -> String;
    }


    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
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

    let tweet = Tweet {
        username: String::from("Sato"),
        content: String::from("Some call it Karma, but to me it's the self inflicted harm that does the job."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // Default trait

    pub trait DefaultSummary {
        fn default_summarize(&self) -> String {
            String::from("Read more ...")
        }
    }

    impl DefaultSummary for NewsArticle {}

    let article = NewsArticle {
        headline: String::from("Trying to hit the impression"),
        content: String::from("Stop trying to have a good impression if you want impression"),
        location: String::from("Anbar"),
        author: String::from("Sato"),

    };

    println!("New article is out: {}", article.default_summarize());


}
