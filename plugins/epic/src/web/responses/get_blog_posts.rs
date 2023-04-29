use std::time::SystemTime;

// This isn't everything since I'm not going to use all of it unless I add a browser element or html parsing
// for the fields that are HTML
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlogItem {
    image: String,               // smaller 1:1
    share_image: Option<String>, // 16:9 wide
    trending_image: String,      // 19:6 wide af
    author: String,
    title: String,
    date: SystemTime,
    external_link: Option<String>,
    slug: String,
    #[serde(default, rename = "_metaTags")]
    meta_tags: String,
    #[serde(default)]
    share_description: String,
    // content: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetBlogPosts {
    blog_list: Vec<BlogItem>,
    blog_total: i32,
    post_count: i32,
    increment_count: i32,
    articles_to_load: i32,
}
