use chrono::{DateTime, Utc};

// This isn't everything since I'm not going to use all of it unless I add a browser element or html parsing
// for the fields that are HTML
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlogItem {
    pub image: String,               // smaller 1:1
    pub share_image: Option<String>, // 16:9 wide
    pub trending_image: String,      // 19:6 wide af
    pub author: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub external_link: Option<String>,
    pub slug: String,
    #[serde(default, rename = "_metaTags")]
    pub meta_tags: String,
    #[serde(default)]
    pub share_description: String,
    // pub content: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBlogPosts {
    pub blog_list: Vec<BlogItem>,
    pub blog_total: i32,
    pub post_count: i32,
    pub increment_count: i32,
    pub articles_to_load: i32,
}
