extern crate reqwest;
extern crate scraper;
extern crate url;

pub fn path_filter(href: &str) -> bool {
    let pat: &'static str = "/";
    !href.starts_with(pat) || href == pat
}

fn main() {
    let urls: Vec<String> = std::env::args().skip(1).collect();
    let mut paths: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    for url in urls {
        eprintln!("Calling {}", url);
        let input: &str = url.as_str();
        let _url: url::Url = url::Url::parse(input).expect("Could not parse url");

        let url: &str = url.as_str();
        let document: String = reqwest::get(url)
            .expect("Request failed")
            .text()
            .expect("Body failed");
        let document: &str = document.as_str();

        let doc: scraper::Html = scraper::Html::parse_document(document);
        let selector: scraper::Selector =
            scraper::Selector::parse("a[href]")
                .expect("Failed to parse links");

        for element in doc.select(&selector) {
            let href: &str =
                element.value().attr("href").expect("Could not find attr{href}");

            if path_filter(href) {
                continue;
            }

            let value: String = href.to_string().clone();
            paths.insert(value);
        }
    }

    for s in &paths {
        println!("{}", s);
    }

    eprintln!("{}", paths.len());
}
