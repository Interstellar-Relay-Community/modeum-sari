use super::FilterConfig;
use once_cell::sync::Lazy;
use regex::Regex;
use sonic_rs::{pointer, JsonValueTrait, Value};
use std::sync::Arc;

static FILTERS: Lazy<Arc<Vec<Regex>>> = Lazy::new(|| {
    let re = vec![
        // GTUBE
        Regex::new(
            r"XJS\*C4JDBQADN1\.NSBN3\*2IDNEN\*GTUBE-STANDARD-ANTI-UBE-TEST-ACTIVITYPUB\*C\.34X",
        )
        .unwrap(),
        // Sorry, @ap12, but they are using your name in spam
        Regex::new(r"mastodon-japan\.net\/@ap12").unwrap(),
        // Would you kindly stop spamming in Korean?
        Regex::new(r"한국괴물군").unwrap(),
        // Fucking discord.
        Regex::new(r"discord.gg\/ctkpaarr").unwrap(),
        // Fucking troll japanese message
        Regex::new(r"画像が貼れなかったのでメンションだけします").unwrap(),
        // Are they in civil war?
        Regex::new(r"荒らし.com").unwrap(),
        Regex::new(r"ctkpaarr.org").unwrap(),
    ];
    Arc::new(re)
});

pub async fn filter(object: &Value, _filter_config: &FilterConfig) -> Result<(), ()> {
    // Extract content
    if let Some(content_str) = object.pointer(pointer!["content"]).as_str() {
        for re in FILTERS.iter() {
            if re.captures(content_str).is_some() {
                return Err(());
            }
        }
    }
    Ok(())
}
