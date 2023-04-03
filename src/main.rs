use gen_core::OpenApiSpec;
use gen_reqwest::{Builder, Config};
use std::fs;

fn main() -> Result<(), anyhow::Error> {
    let file = fs::read_to_string("example.json").unwrap();
    let spec = serde_json::from_str::<OpenApiSpec>(file.as_str()).expect("error in json file");

    let builder = Builder::new(Config { out_dir: "./tmp" }, spec);

    builder.generate()
}
