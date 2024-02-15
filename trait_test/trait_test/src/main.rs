use aggregator::{self, Summary, NewsArticle, Tweet};

/*fn notify(item: impl Summary) {
    println!("notify: {}",  item.summarize())
}*/

pub fn notify<T: Summary>(item: &T) {
    println!("頭條新聞！{}", item.summarize());
}

fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    //println!("1 則新推文：{}",tweet.summarize());

    notify(&tweet);
    /*let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };*/

    //println!("有新文章發佈！{}", article.summarize());
}
