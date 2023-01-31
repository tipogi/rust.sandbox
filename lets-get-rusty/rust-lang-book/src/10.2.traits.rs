// TRAITS
// We define the shared behavior between a tweet and a news article
// which in this case is `summarization`. 
// Shared behaivour methods, traits, allow us to define a set of methods
// that are shared across different types 
// We can define just the method signature, it does not have a method body. We do not
// want to dictate the implementation. Also, the trait can have a default method implementation
// Every type that implements the trait, it has all that function(s). Some already implemented or anothers
// waiting for implementation

use std::fmt::Debug;
use std::fmt::Display;

pub struct NewsArticle {
  pub author: String,
  pub headline: String,
  pub content: String,
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

// We will define a shared methods. Similar interface
pub trait Summary {
  fn summarize(&self) -> String;
  // Also we have to option to implement the body and make it as default
  fn summarize_default(&self) -> String {
    format!("(Read more...)? YES: {}", self.summarize_author())
  }
  fn summarize_author(&self) -> String;
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {}", self.headline, self.summarize_author())
  }
  fn summarize_author(&self) -> String {
      format!("{}", self.author)
  }
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("{}, by {}", self.username, self.content)
  }
  fn summarize_author(&self) -> String {
      format!("@{}", self.username)
  }
  fn summarize_default(&self) -> String {
    format!("Extended info of the Tweet: {}, by {}", self.summarize_author(), self.content)
  }
}

// Traits as Parameter. As an arguments the implements Summary trait
/*pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}*/

// The same as above, another way to define. We call to that TRAIT BOUNDS
// The generic element is limited to 'Summary' traits
// When is simple function the above code is better but when it has more arguments and the same type,
// it is better to use TRAIT BOUNDS. Check below
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

pub fn notify_b<T: Summary>(item_a: &T, item_b: &T) {
  println!("Breaking news! {}", item_a.summarize());
}

/*pub fn notify_b(item_a: &impl Summary, item_b: &impl Summary) {
  println!("Breaking news! {}", item_a.summarize());
}*/

pub fn notify_c<T: Summary + Display >(item_a: &T, item_b: &T) {
  println!("Breaking news! {}", item_a.summarize());
}

/*pub fn notify_c(item_a: &(impl Summary + Display), item_b: &(impl Summary + Display)) {
  println!("Breaking news! {}", item_a.summarize());
}*/

// Use where clause to define multiple traits
fn random_fn<T, U>(t: &T, u: &U) -> i32
  // T implements Display and clone and U implements Clone and Debug
  where T: Display + Clone,
        U: Clone + Debug
{
  21
}

fn tweet_summarise() {
  let tweet = Tweet {
    username: String::from("rusty"),
    content: String::from("Hello rusty!"),
    reply: false,
    retweet: false
  };

  let article = NewsArticle {
    author: String::from("John Dow"),
    headline: String::from("The sky is falling!"),
    content: String::from("The sky is not actually falling")
  };

  println!("Tweet summary {}", tweet.summarize());
  println!("Article summary {}", article.summarize());
  println!("Tweet summary {}", tweet.summarize_default());
  println!("Article summary {}", article.summarize_default());

  notify(&article);
}


// LETS CHECK RETURN TYPES
// Returning types that implement a certain trait instead of concretate types, is very useful in closures and iterators
// That return type can just return one type element. It cannot be if a == 2 Tweet else Article
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("Nostr"),
    content: String::from("Open all the squares for speech!"),
    reply: true,
    retweet: false
  }
}

//
struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  // That function is going to be just available to pair structs where the type of x and y implement
  // Display and PartialOrd 
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest number is x = {}", self.x);
    } else {
      println!("The largest number is y = {}", self.y);
    }
  }
}

// Blanket implementations
// https://youtu.be/T0Xfltu4h3A?t=627

fn main() {
  tweet_summarise();
  println!("{}", returns_summarizable().summarize());
  let pair_a = Pair::new(
    String::from("hello"), 
    String::from("rust")
  );
  pair_a.cmp_display();
  let wrong_pair = Pair::new(false, true);
  wrong_pair.cmp_display();
}