use askama::Template;

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "data.html", escape = "none")]
pub struct DataTemplate {}