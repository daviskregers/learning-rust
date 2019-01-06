
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn number(&self) -> f32 {
        self.x
    }
}
impl Point<i32> {
    fn number(&self) -> i32 {
        self.x
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({}) \n {}", self.headline, self.author, self.location, self.content)
    }
}

fn main() {
    
    let list = vec![23,54,65,67];
    let result = largest(&list);
    println!("{}", result);

    let list = vec![223,544,655,67];
    let result = largest(&list);
    println!("{}", result);

    let list = vec!['y','t','u'];
    let result = largest(&list);
    println!("{}", result);

    let integer = Point{x:5, y:10};
    let float = Point{x:8.0, y:9.4};
    println!("{:?}\n{:?}", integer, float);
    println!("{:?}", integer.x());
    println!("{:?}", float.x());

    let n = Point{x:2.2, y: 3.14};
    println!("{}", n.number());
    let n = Point{x:2, y: 3};
    println!("{}", n.number());

    // Traits

    let news = NewsArticle {
        headline: String::from("The title"),
        location: String::from("The location"),
        author: String::from("The author"),
        content:String::from("The content"),
    };

    println!("News Article \n{}", news.summarize());

}

fn largest<T:PartialOrd+Copy>(list :&[T]) -> T {
    let mut largest = list[0];
    for n in list {
        if n > &largest {
            largest = *n;
        }
    }
    largest
}

