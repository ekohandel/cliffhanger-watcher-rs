use crate::notifier::Notifier;
use crate::scraper::Scraper;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub struct Watcher {
    notifier: Notifier,
    numbers: Vec<String>,
    scraper: Scraper,
    spots: HashMap<String, u32>,
}

impl Watcher {
    const DATA_FILE: &'static str = "/data/spots.json";
    const NOTIFICATION_TEXT: &'static str =
        "Cliffhanger Watcher (rs): New spots are available, better get booking!";

    pub fn new(notifier: Notifier, numbers: Vec<String>) -> Self {
        let mut content = String::new();
        if let Ok(mut file) = OpenOptions::new().read(true).open(Self::DATA_FILE) {
            file.read_to_string(&mut content)
                .expect("Failed to read data file content");
        }

        Watcher {
            notifier,
            numbers,
            scraper: Scraper::new(),
            spots: serde_json::from_str(&content).unwrap_or_default(),
        }
    }

    pub async fn update(&mut self) {
        let current_sports = self.scraper.get_times();
        for (spot_time, _) in current_sports.iter() {
            if self.spots.get_key_value(spot_time).is_none() {
                self.notifier
                    .notify(&self.numbers, Self::NOTIFICATION_TEXT)
                    .await;

                current_sports.iter().for_each(|(k, v)| {
                    self.spots.insert(String::from(k), *v);
                });

                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(Path::new(Self::DATA_FILE))
                    .expect("Could not open data file for write")
                    .write_all(
                        serde_json::to_string(&self.spots)
                            .unwrap_or_default()
                            .as_bytes(),
                    )
                    .expect("Could not write content to data file");

                break;
            }
        }
    }
}
