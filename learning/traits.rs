trait Summary {
    fn by(&self) -> String;

    fn summarize(&self) -> String {
        let mut summary = String::new();
        summary.push_str("...read more\n");
        summary.push_str(&format!("by {}", self.by())[..]);
        summary
    }
}

trait Dummy {}

struct Post {
    user: String,
    content: String,
}

impl Summary for Post {
    fn by(&self) -> String {
        format!("@{}", self.user)
    }
}

struct News {
    head: String,
    author: String,
    content: String
}

impl Summary for News {
    fn by(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.head, self.by())
    }
}

impl Dummy for Post {}



fn main() {
    let post = Post {
        user: String::from("abhinav"),
        content: String::from("content of the post")
    };

    let news = News {
        head: String::from("News Heading"),
        author: String::from("abhinav"),
        content: String::from("content of the news")
    };

    print_summary(&post);
    print_summary2(&news);
    print_summary3(&news, &post);
    // print_summary4(&post, &news);
    print_summary4(&post, &post);
}



fn print_summary<T: Summary> (item: &T) {
    println!("Summary:");
    println!("{}\n", item.summarize());
}

fn print_summary2(item: &impl Summary) {
    println!("Summary:");
    println!("{}\n", item.summarize());
}

fn print_summary3(item1: &impl Summary, item2: &(impl Summary + Dummy)) {
    println!("Summary:");
    println!("{}\n", item1.summarize());
    println!("{}\n", item2.summarize());
}

fn print_summary4<T, U> (item1: &T, item2: &U)
where
    T: Summary,
    U: Summary + Dummy,
{
    println!("Summary:");
    println!("{}\n", item1.summarize());
    println!("{}\n", item2.summarize());
}