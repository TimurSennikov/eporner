use eporner::r34_api::r34_api::R34;
use eporner::save_tools::save_tools;
use eporner::arg_taker::arg_taker;

fn main() {
    let client = reqwest::Client::new();

    let tags = arg_taker::get_arg("tags").expect("Please specify 'tags (tag1,tag2,tag3...)' as a pair of input parameters.");
    let exclude = match arg_taker::get_arg("exclude") {
        Ok(e) => e,
        Err(_) => "".to_string()
    };

    let output = arg_taker::get_arg("output").expect("Please specify 'out (out_directory)' as a pair of input parameters.");
    let outfold = std::path::Path::new(&output);
    if !outfold.exists() {
        println!("Directory not found, trying to create...");
        if let Err(_) = std::fs::create_dir(outfold) {
            panic!("Failed to create directory {}", output);
        }
    }

    let tags: Vec<&str> = tags.split(",").collect();
    let exclude: Vec<&str> = exclude.split(",").collect();

    println!("Retrieving list of files to download. This can take a LOT of time if the tag is popular.");
    let files = R34::query(&tags, &exclude, 0);

    println!("Found {} posts.", files.len());

    trpl::run(async {
        for post in files {
            println!("Saving {}...", post.filename);
            save_tools::save(&client, &post, &output).await;
        }
    });

    println!("All jobs done. Check {} for output.", output);
}
