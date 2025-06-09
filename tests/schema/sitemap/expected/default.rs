pub type Urlset = UrlsetType;
#[derive(Debug)]
pub struct UrlsetType {
    pub url: Vec<TUrlType>,
}
#[derive(Debug)]
pub struct TUrlType {
    pub loc: String,
    pub lastmod: Option<String>,
    pub changefreq: Option<TChangeFreqType>,
    pub priority: Option<f64>,
}
#[derive(Debug)]
pub enum TChangeFreqType {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}
