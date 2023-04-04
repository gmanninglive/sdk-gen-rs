use gen_core::{BuilderConfig, BuilderTrait, Method, OpenApiSpec, Request};
use rust_embed::RustEmbed;
use std::fs;

#[derive(RustEmbed)]
#[folder = "./templates"]
struct Template;

#[derive(Debug, Clone)]
pub struct Builder {
    config: BuilderConfig,
    spec: OpenApiSpec,
}

impl BuilderTrait for Builder {
    fn new(config: BuilderConfig, spec: OpenApiSpec) -> Self {
        Self { config, spec }
    }
    fn generate(self) -> Result<(), anyhow::Error> {
        generate_crate(self.config.clone());

        let mut generated_requests: String = Default::default();

        for path in self.spec.paths.into_iter() {
            let (url, requests) = path;

            for req in requests.iter() {
                match req.0 {
                    Method::GET => {
                        generated_requests += "\t";
                        generated_requests += generate_get(BuildReq {
                            url: url.clone(),
                            request: req.1.clone(),
                        })
                        .as_str();
                    }
                    _ => {}
                }
            }
        }

        let generated_lib =
            String::from_utf8(Template::get("lib.handlebars").unwrap().data.to_vec())
                .expect("error reading template")
                .replace("{{REQUESTS}}", &generated_requests);

        fs::create_dir_all(self.config.out_dir.join("src"))?;
        fs::write(self.config.out_dir.join("src/lib.rs"), generated_lib)?;
        Ok(())
    }
}

struct BuildReq {
    url: String,
    request: Request,
}

fn generate_get(build: BuildReq) -> String {
    let url = build.url;
    let params = build.request.parameters;
    let operation_id = build.request.operation_id;

    match params {
        Some(_) => format!(
            r#"pub async fn {operation_id}(params: &Params) -> Result<Response, Error> {{
        reqwest::Client::new().get("{url}").query(params).send().await
    }}

"#
        ),
        None => format!(
            r#"pub async fn {operation_id}() -> Result<Response, Error> {{
        reqwest::get("{url}").await
    }}

"#
        ),
    }
}

fn generate_crate(config: BuilderConfig) {
    let toml = r#"[package]
name = "generated-sdk"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
reqwest = "*"
"#;

    fs::write(config.out_dir.join("Cargo.toml"), toml).expect("error writing Cargo.toml");
}

#[cfg(test)]
mod tests {
    use gen_core::Response;

    use super::*;

    #[test]
    fn generates_get_request_without_params() {
        let result = generate_get(BuildReq {
            url: "/test".to_owned(),
            request: Request {
                description: "test description".to_owned(),
                operation_id: "test_request".to_owned(),
                responses: Response {},
                summary: "test summary".to_owned(),
                tags: vec!["get request".to_owned()],
                parameters: None,
            },
        });

        let expected = "    pub async fn test_request() -> Result<Response, Error> {\n        reqwest::get(\"/test\").await\n    }\n\n".to_owned();
        assert_eq!(result, expected);
    }
}
