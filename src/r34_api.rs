pub mod r34_api {
    use json::JsonValue;

    pub struct PostData{
        pub url: String,
        pub filename: String
    }

    pub struct R34{}

    impl R34{
        pub fn query(tags: &Vec<&str>, exclude: &Vec<&str>, page: u32) -> Vec<PostData> {
            let url = format!("https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&json=1&limit=1000&pid={}&tags={}", page, tags.join("%20"));

            let req = reqwest::blocking::get(url).expect("Failed to send a request to rule 34 api.");
            let text = req.text().expect("Failed to get text from api answer.");

            let parsed = json::parse(&text).expect("Invalid json retrieved from rule 34 api.");

            let mut urls: Vec<PostData> = vec![]; 

            let members = parsed.members();

            let members_filtered: Vec<&JsonValue> = members.filter(|member| {
                !exclude.iter().any(|v|{member["tags"].to_string().contains(v) && v.len() > 0})
            }).collect();

            for elem in &members_filtered {
                urls.push(PostData{url: elem["file_url"].to_string(), filename: elem["image"].to_string()});
            }

            if members_filtered.len() == 1000 {
                println!("Detected more pages. Fetching page {}", page+1);

                let mut next_page = Self::query(tags, exclude, page+1);
                urls.append(&mut next_page);
            }

            urls
        }
    }
}
