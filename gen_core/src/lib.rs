use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, serde::Deserialize)]
struct Contact {
    email: String,
    name: String,
}
#[derive(Debug, serde::Deserialize)]
pub struct License {
    name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct InfoObject {
    contact: Contact,
    description: String,
    license: License,
    title: String,
    version: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct OpenApiSpec {
    info: InfoObject,
    pub paths: HashMap<String, HashMap<Method, Request>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Paths {
    get: Option<Request>,
    post: Option<Request>,
}

#[derive(Debug, serde::Deserialize, Eq, PartialEq, Hash)]
pub enum Method {
    #[serde(rename(deserialize = "get"))]
    GET,
    #[serde(rename(deserialize = "post"))]
    POST,
    #[serde(rename(deserialize = "put"))]
    PUT,
    #[serde(rename(deserialize = "delete"))]
    DELETE,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Response {}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Request {
    pub description: String,
    #[serde(rename(deserialize = "operationId"))]
    pub operation_id: String,
    pub responses: Response,
    pub summary: String,
    pub tags: Vec<String>,
    pub parameters: Parameters,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct SchemaObj {
    r#type: String,
}

pub type Parameters = Option<Vec<Parameter>>;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Parameter {
    description: String,
    r#in: ParameterLocation,
    name: String,
    required: bool,
    schema: SchemaObj,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    Path,
    Query,
    Header,
    Cookie,
}

#[derive(Clone)]
pub struct BuilderConfig {
    pub out_dir: PathBuf,
}

pub trait BuilderTrait {
    fn new(config: BuilderConfig, spec: OpenApiSpec) -> Self;
    fn generate(self) -> Result<(), anyhow::Error>;
}
