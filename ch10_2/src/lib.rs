//traits

/*
pub trait Summary{
    fn summarize(&self) -> String
}
*/
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
        //this is a default implementation
        //if this trait is implemented for a 
        //type but no functionn body is filled, it will resort
        //to this call. 
        //after the type, you'd write:
        //impl Summary for Type{}
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
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