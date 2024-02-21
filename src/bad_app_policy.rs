use super::FilterConfig;
use sonic_rs::{pointer, JsonValueTrait, Value};

pub async fn filter(object: &Value, _filter_config: &FilterConfig) -> Result<(), ()> {
    if let Some(app_name) = object.pointer(pointer!["application", "name"]).as_str() {
        if let Some(username) = object.pointer(pointer!["account", "username"]).as_str() {
            if app_name == username {
                return Err(());
            }
        }
    }

    Ok(())
}
