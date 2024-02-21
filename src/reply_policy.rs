use super::FilterConfig;
use sonic_rs::{pointer, JsonContainerTrait, JsonValueTrait, Value};

pub async fn filter(object: &Value, filter_config: &FilterConfig) -> Result<(), ()> {
    if let Some(reply_to) = object.pointer(pointer!["inReplyTo"]) {
        if reply_to.is_null() {
            if let Some(tags) = object.pointer(pointer!["tag"]).as_array() {
                let mut mention_cnt = 0;
                for tag in tags.iter() {
                    if let Some(tag_type) = tag.pointer(pointer!["type"]).as_str() {
                        if tag_type == "Mention" {
                            mention_cnt += 1;
                        }
                    }
                }

                if mention_cnt >= filter_config.max_new_reply_cnt {
                    return Err(());
                }
            }
        }
    }

    Ok(())
}
