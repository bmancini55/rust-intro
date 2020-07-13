use std::fmt::Debug;
use std::fmt::Display;

/// A trait can have more than a single method signature in it. We use
/// trait bounds to enable access to similar behavior across various
/// structs.
///
/// One restriction to note with trait implementations is that we can
/// implement a trait on a type only if either the trait or the type is
/// local to our crate.
///
/// A method on a Trait can have a default as shown below. It can also
/// have no implementation and require each class that has the trait
/// to implement the method.
/// ```
/// pub trait Summary {
///     fn summarize(&self) -> String;
/// }
/// ```
///
/// Below is an example of a default trait that can be override by an
/// implementing class.
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

/// Implements the Summary Trait for the Tweet struct
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/// Implements the Summary Trait for the NewsArticle using the default
/// methods
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

/// Implements the Display Trait for News Article
impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.headline, self.author, self.location,)
    }
}

/// We can use a trait by directly referencing the Trait. This techqniue
/// works for straighforward cases but gets very verbose quickly.
pub fn notify_direct(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// Instead of using &impl Summary as the type, we can use a Generic
/// that is bound to the Trait instead. This is cleaner syntax
pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// We can also use trait binding to enforce the exact same type for
/// more than one argument. Using the direct syntax we can only enforce
/// that both arguments support our Trait
pub fn notify_multi<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

/// We can also combine multiple Traits using +.  This can be used in the
/// direct syntax or in Trait Bound syntax.
pub fn notify_display<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// Using too many trait bouunds can get pretty crazy. When this happens
/// You can instead use where clauses
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

/// We can also return traits using `impl Trait` syntax as the return
/// type. This can ONLY be done when it returns a single type. Doing
/// multiple type returns is covered in a later section.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    // The summarize trait was implemented for both Tweet and NewArticle
    // and we can use the method on either one.
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // Example of the summary being implemented with a default trait
    // method.
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    // Argument accepts any type by the trait name. This technique works
    // for simple cases.
    notify_direct(&tweet);

    // Argument uses trait binding instead of the trait name
    notify_bound(&tweet);

    // Using trait binding for multiple arguments at the same time. We
    // cannot pass both a Tweet and a news Article because the trait
    // binding enforces a single Type
    notify_multi(&tweet, &tweet);

    // Requires that our type implement both Summary and Display
    notify_display(&article);

    // Constructs an object that implements the Summarizable trait
    // and use it with one of our methods that accept a Summarizable
    // type
    let summarizable = returns_summarizable();
    notify_bound(&summarizable);
}
