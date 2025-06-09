use core::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
pub type Urlset = UrlsetType;
#[derive(Debug, Serialize, Deserialize)]
pub struct UrlsetType {
    #[serde(default, rename = "url")]
    pub url: Vec<TUrlType>,
}
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct TChangeFreqType {
    #[serde(rename = "#text")]
    pub value: TChangeFreqValue,
}
impl From<TChangeFreqValue> for TChangeFreqType {
    fn from(value: TChangeFreqValue) -> Self {
        Self { value }
    }
}
impl From<TChangeFreqType> for TChangeFreqValue {
    fn from(value: TChangeFreqType) -> Self {
        value.value
    }
}
impl Deref for TChangeFreqType {
    type Target = TChangeFreqValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl DerefMut for TChangeFreqType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TChangeFreqValue {
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
