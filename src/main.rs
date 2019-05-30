extern crate reqwest;
extern crate scraper;

fn main() {
    let urls: Vec<String> = std::env::args().skip(1).collect();
    let mut paths: std::collections::HashSet<&str> =
        std::collections::HashSet::new();

    for url in urls {
        println!("Calling {}", url);
        let url: &str = url.as_str();
        let document: String = reqwest::get(url)
            .expect("Request failed")
            .text()
            .expect("Body failed");
        let document: &str = document.as_str();

        let doc: scraper::Html = scraper::Html::parse_document(document);
        let select: scraper::Selector =
            scraper::Selector::parse("a[href]")
                .expect("Failed to parse links");

        for element in doc.select(&select) {
            let href: &str =
                element.value().attr("href").expect("Could not find attr{href}");

            let pat: &'static str = "/";
            if !href.starts_with(pat) {
                continue;
            }
        }
    }

    for s in paths {
        println!("{}", s);
    }

    println!("# of paths {}", paths.len());
}
