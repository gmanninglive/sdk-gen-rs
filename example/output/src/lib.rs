use reqwest::{Response, Error};

struct SDK {}

impl SDK {
	pub async fn list_todos() -> Result<Response, Error> {
        reqwest::get("/todo").await
    }

	pub async fn search_todos(params: &Params) -> Result<Response, Error> {
        reqwest::Client::new().get("/todo/search").query(params).send().await
    }


}
