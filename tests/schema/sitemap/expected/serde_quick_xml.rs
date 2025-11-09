use serde::{Deserialize, Serialize};
pub type Urlset = UrlsetType;
#[derive(Debug, Deserialize, Serialize)]
pub struct UrlsetType {
    #[serde(default, rename = "url")]
    pub url: Vec<TUrlType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TUrlType {
    #[serde(rename = "loc")]
    pub loc: String,
    #[serde(default, rename = "lastmod")]
    pub lastmod: Option<String>,
    #[serde(default, rename = "changefreq")]
    pub changefreq: Option<TChangeFreqType>,
    #[serde(default, rename = "priority")]
    pub priority: Option<f64>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum TChangeFreqType {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "yearly")]
    Yearly,
    #[serde(rename = "never")]
    Never,
}
