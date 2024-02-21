use sonic_rs::Value;

mod bad_app_policy;
mod blurhash_policy;
mod regex_policy;
mod reply_policy;

pub struct FilterConfig {
    pub max_new_reply_cnt: i32,
}

pub async fn filter(object: &Value, filter_config: &FilterConfig) -> Result<(), usize> {
    let results = [
        regex_policy::filter(object, filter_config).await,
        reply_policy::filter(object, filter_config).await,
        blurhash_policy::filter(object, filter_config).await,
        bad_app_policy::filter(object, filter_config).await,
    ];

    for (i, &result) in results.iter().enumerate() {
        match result {
            Ok(_) => {}
            Err(_) => {
                return Err(i);
            }
        }
    }

    Ok(())
}
