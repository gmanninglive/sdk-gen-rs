use std::fs;

use gen_core::{Method, OpenApiSpec, Request};

pub struct Config {
    pub out_dir: &'static str,
}

pub struct Builder {
    config: Config,
    spec: OpenApiSpec,
}

impl Builder {
    pub fn new(config: Config, spec: OpenApiSpec) -> Self {
        Self { config, spec }
    }
    pub fn generate(self) -> Result<(), anyhow::Error> {
        let mut string_builder: String = Default::default();
        for path in self.spec.paths.into_iter() {
            let (url, requests) = path;

            for req in requests.iter() {
                match req.0 {
                    Method::GET => {
                        string_builder += generate_get(BuildReq {
                            url: url.clone(),
                            request: req.1.clone(),
                        })
                        .as_str();
                    }
                    _ => {}
                }
            }
        }

        fs::write(self.config.out_dir, string_builder)?;
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
            r#"pub async fn {operation_id}(params: Params) -> Result<Response, Error> {{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
