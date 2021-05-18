use cliffhanger_watcher::notifier::Notifier;
use cliffhanger_watcher::watcher::Watcher;
use std::{env, thread, time};
use tokio::runtime::Runtime;

fn main() {
    let mut watcher = Watcher::new(Notifier::new(), vec![String::from("+14034797856")]);
    let update_interval = time::Duration::from_secs(
        env::var("CLIFFHANGER_WATCHER_UPDATE_INTERVAL")
            .unwrap_or_default()
            .parse()
            .unwrap(),
    );

    loop {
        Runtime::new().unwrap().block_on(watcher.update());
        thread::sleep(update_interval);
    }
}
