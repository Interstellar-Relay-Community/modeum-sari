use super::FilterConfig;
use sonic_rs::{pointer, JsonContainerTrait, JsonValueTrait, Value};

static BAD_BLURHASHES: [&str; 4] = [
    "UTQcbkVY%gIU8w8_%Mxu%2Rjayt7.8?bMxRj", // discord link in 5000yen style
    "UTQcblVY%gIU8w8_%Mxu%2Rjayt7.8?bMxRj",
    "UkK2FEk8Oas:t1f9V[ae|;agoJofs;bYowjZ", // Spam image
    "UkK2FFk8Oas:tKf9V[ae|;agoJoft7bYovjZ",
];

pub async fn filter(object: &Value, _filter_config: &FilterConfig) -> Result<(), ()> {
    if let Some(attachments) = object.pointer(pointer!["attachment"]).as_array() {
        // It has an attachment.
        for attachment in attachments.iter() {
            if let Some(blurhash) = attachment.pointer(pointer!["blurhash"]).as_str() {
                for bad_hash in BAD_BLURHASHES.iter() {
                    if *bad_hash == blurhash {
                        // Spam!!
                        return Err(());
                    }
                }
            }
        }
    }

    Ok(())
}
