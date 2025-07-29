trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        // Ensure we don't slice beyond content length
        let end = std::cmp::min(50, self.content.len());
        format!("{}....", &self.content[0..end])
    }
}

fn main() {
    // Create a sample article
    let article = Article {
        headline: String::from("Breaking News: Rust Programming"),
        content: String::from("Rust is a modern systems programming language focused on safety and performance."),
    };

    // Print the headline and summary
    println!("Headline: {}", article.headline);
    println!("Summary: {}", article.summarize());

    // Test with a short content article
    let short_article = Article {
        headline: String::from("Short News"),
        content: String::from("Brief news."), 
    };

    println!("\nShort Article");
    println!("Headline: {}", short_article.headline);
    println!("Summary: {}", short_article.summarize());
    
    println!("\nArticle");
    println!("Headline: {}", article.headline);
    println!("Summary: {}", article.summarize());
}
