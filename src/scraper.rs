use scraper::{Html, Selector};

pub struct Scraper {
    url: &'static str,
}

impl Scraper {
    pub fn new() -> Self {
        Scraper {
            url: "https://cliffhangerclimbing.com/core/book/member-booking",
        }
    }

    fn get_times_list(self) -> Vec<String> {
        if let Ok(request) = reqwest::blocking::get(self.url) {
            if let Ok(text) = request.text() {
                let document = Html::parse_document(&text);
                let tbody_selector = Selector::parse("tbody").unwrap();
                let label_selector = Selector::parse("label").unwrap();

                if let Some(table) = document.select(&tbody_selector).next() {
                    return table
                        .select(&label_selector)
                        .map(|x| x.inner_html())
                        .map(|x| String::from(x.trim()))
                        .collect();
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("{:?}", Scraper::new().get_times_list());
    }
}
