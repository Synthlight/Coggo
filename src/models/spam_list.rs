pub const SPAM_LIST_URL: &str = "https://raw.githubusercontent.com/DevSpen/links/master/src/links.txt";

pub struct SpamList {
    entries: Option<Vec<String>>,
}

impl SpamList {
    pub fn new() -> Self {
        return SpamList {
            entries: None,
        };
    }

    pub async fn setup(&mut self) {
        if self.entries.is_none() {
            println!("Fetching/caching spam list from: {}", SPAM_LIST_URL);

            let client = reqwest::Client::new();
            let body = client.get(SPAM_LIST_URL)
                .send().await.expect("Error getting spam list.")
                .text().await.expect("Error getting spam list.");
            let split = body.lines();
            let vec = split.map(|s| s.to_string()).collect::<Vec<String>>();

            self.entries = Some(vec);

            // for x in self.get_contents() {
            //     println!("{}", x);
            // }

            println!("Spam list cached.");
        }
    }

    pub fn get_contents(&self) -> Vec<String> {
        return self.entries.clone().unwrap();
    }
}