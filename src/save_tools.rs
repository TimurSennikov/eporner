pub mod save_tools{
    use std::io::Write;

    pub async fn save(client: &reqwest::Client, elem: &crate::r34_api::r34_api::PostData, path: &str){
        let mut res = client.get(&elem.url).send().await.unwrap();

        let mut downloaded = 0;
        let content_len = res.content_length().unwrap();

        let fpath = format!("{}/{}", path, elem.filename);
        if std::path::Path::new(&fpath).exists(){eprintln!("Path {} already exists. Skipping...", fpath); return ();}

        let mut f = std::fs::File::create(fpath).unwrap();

        while let Ok(Some(chunk)) = res.chunk().await {
            downloaded += chunk.len();

            print!("|_ Downloaded {} out of {} megabytes.\r", downloaded / 1000 / 1000, content_len / 1000 / 1000);

            match f.write_all(&chunk) {
                Ok(_) => {},
                Err(_) => {eprintln!("Error writing to file {}. Continuing...", elem.filename); return ();}
            }
        }

        println!("");
    }
}
