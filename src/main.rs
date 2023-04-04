use std::{fs, path::PathBuf};

use clap::{arg, command, value_parser};
use gen_core::{BuilderConfig, BuilderTrait, OpenApiSpec};
use gen_reqwest::*;

fn main() -> Result<(), anyhow::Error> {
    let matches = command!()
        .arg(
            arg!(
                -i --input <FILE> "Sets input openapi json file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --out_dir <DIR> "Sets output directory"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let spec_path = matches
        .get_one::<PathBuf>("input")
        .expect("input file path missing");
    let out_dir = matches
        .get_one::<PathBuf>("out_dir")
        .expect("output directory path missing");
    let file = fs::read_to_string(spec_path).unwrap();
    let spec =
        serde_json::from_str::<OpenApiSpec>(file.as_str()).expect("error deserializing json");

    let builder = Builder::new(
        BuilderConfig {
            out_dir: out_dir.clone(),
        },
        spec,
    );

    if !out_dir.exists() {
        fs::create_dir_all(out_dir)?;
    }

    builder.generate()?;

    Ok(())
}
