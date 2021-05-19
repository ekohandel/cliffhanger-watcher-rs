use cliffhanger_watcher::notifier::Notifier;
use cliffhanger_watcher::watcher::Watcher;
use std::{env, thread, time};
use tokio::runtime::Runtime;

fn main() {
    let numbers: Vec<String> = env::var("CLIFFHANGER_WATCHER_NUMBERS")
        .expect("Did not find CLIFFHANGER_WATCHER_NUMBERS environment variable")
        .split(';')
        .map(String::from)
        .collect();
    let mut watcher = Watcher::new(Notifier::new(), numbers);
    let update_interval = time::Duration::from_secs(
        env::var("CLIFFHANGER_WATCHER_UPDATE_INTERVAL")
            .expect("Did not find CLIFFHANGER_WATCHER_UPDATE_INTERVAL environment variable")
            .parse()
            .expect("Could not parse CLIFFHANGER_WATCHER_UPDATE_INTERVAL"),
    );

    loop {
        Runtime::new().unwrap().block_on(watcher.update());
        thread::sleep(update_interval);
    }
}
