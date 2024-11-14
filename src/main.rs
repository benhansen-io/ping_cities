use std::borrow::Cow;

use serde::Deserialize;
use skim::prelude::*;

#[derive(Debug, Deserialize, Clone)]
struct Endpoint {
    name: String,
    domain: String,
}

impl SkimItem for Endpoint {
    fn text(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

const ENDPOINTS_DATA: &str = include_str!("../endpoints.json");

fn main() {
    let endpoints: Vec<Endpoint> =
        serde_json::from_str(ENDPOINTS_DATA).expect("should be able to parse endpoints.json");

    loop {
        let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
        for endpoint in &endpoints {
            let _ = tx_item.send(Arc::new(endpoint.clone()));
        }
        drop(tx_item); // so that skim could know when to stop waiting for more items.

        let output = Skim::run_with(&SkimOptions::default(), Some(rx_item)).unwrap();

        if output.is_abort {
            return;
        }
        let Some(item) = output.selected_items.first() else {
            continue;
        };
        let endpoint: Endpoint = (**item)
            .as_any()
            .downcast_ref::<Endpoint>()
            .unwrap()
            .to_owned();

        println!("Pinging {}", endpoint.name);
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::process::Command::new("ping")
            .arg("-c")
            .arg("4")
            .arg(endpoint.domain)
            .status()
            .unwrap();
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}
