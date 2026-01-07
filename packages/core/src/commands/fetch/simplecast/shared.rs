use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastSite {
    pub subdomain: String,
    pub external_website: UrlWrapper,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastAuthors {
    pub collection: Vec<SimplecastAuthor>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastAuthor {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplecastCount {
    pub count: u32,
}
