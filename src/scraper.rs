use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub struct Scraper {
    url: &'static str,
}

impl Scraper {
    pub fn new() -> Self {
        Scraper {
            url: "https://cliffhangerclimbing.com/core/book/member-booking",
        }
    }

    pub fn get_times(&self) -> HashMap<String, u32> {
        Self::get_times_from_list(self.get_times_list())
    }

    fn get_times_list(&self) -> Vec<String> {
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

    fn get_times_from_list(times_list: Vec<String>) -> HashMap<String, u32> {
        let re = Regex::new(r"^(.+ at .+)[\t]{8}\((\d+) spots available\)")
            .expect("Could not create regex");
        times_list
            .iter()
            .map(|x| {
                let cap = re.captures(&x).unwrap();
                (
                    String::from(cap.get(1).unwrap().as_str()),
                    cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                )
            })
            .collect()
    }
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manual_test() {
        let s = Scraper::new();
        println!("{:#?}", s.get_times());
    }
}
