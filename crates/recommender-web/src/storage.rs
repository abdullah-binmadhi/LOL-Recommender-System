use recommender_core::contracts::StoredUserRecord;
use serde_json::Value;

pub const USERS_KEY: &str = "championRecommenderUsers";
pub const RECENT_KEY: &str = "recentChampions";

#[cfg(target_arch = "wasm32")]
pub fn get_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

#[cfg(target_arch = "wasm32")]
pub fn load_users_raw() -> Vec<Value> {
    if let Some(storage) = get_storage() {
        if let Ok(Some(json_str)) = storage.get_item(USERS_KEY) {
            return serde_json::from_str(&json_str).unwrap_or_default();
        }
    }
    Vec::new()
}

#[cfg(target_arch = "wasm32")]
pub fn load_users() -> Vec<StoredUserRecord> {
    load_users_raw()
        .into_iter()
        .filter_map(|val| serde_json::from_value(val).ok())
        .collect()
}

#[cfg(target_arch = "wasm32")]
pub fn append_user(
    full_name: &str,
    email: &str,
    phone: &str,
    age: &str,
    gender: &str,
    experience: &str,
) -> Result<(), String> {
    let storage = get_storage().ok_or("localStorage unavailable")?;
    let mut users = load_users_raw();

    let session_id = format!(
        "session_{}_{:09}",
        js_sys::Date::now() as u64,
        ((js_sys::Math::random() * 1e9) as u64)
    );
    let registration_date = js_sys::Date::new_0()
        .to_iso_string()
        .as_string()
        .unwrap_or_default();

    let new_user = serde_json::json!({
        "fullName": full_name,
        "email": email,
        "phone": if phone.trim().is_empty() { "Not provided" } else { phone },
        "age": age,
        "gender": if gender.trim().is_empty() { "Not specified" } else { gender },
        "experience": if experience.trim().is_empty() { "Not specified" } else { experience },
        "registrationDate": registration_date,
        "sessionId": session_id,
    });

    users.push(new_user);
    let json_str = serde_json::to_string(&users).map_err(|e| e.to_string())?;
    storage
        .set_item(USERS_KEY, &json_str)
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn load_recent_champions() -> Vec<String> {
    if let Some(storage) = get_storage() {
        if let Ok(Some(json_str)) = storage.get_item(RECENT_KEY) {
            return serde_json::from_str(&json_str).unwrap_or_default();
        }
    }
    Vec::new()
}

#[cfg(target_arch = "wasm32")]
pub fn update_recent_champions(new_recommendations: &[String]) -> Result<(), String> {
    let storage = get_storage().ok_or("localStorage unavailable")?;
    let existing = load_recent_champions();
    let updated = recommender_core::export::update_recent_champions(&existing, new_recommendations);
    let json_str = serde_json::to_string(&updated).map_err(|e| e.to_string())?;
    storage
        .set_item(RECENT_KEY, &json_str)
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn trigger_csv_download(filename: &str, content: &str) -> Result<(), String> {
    use wasm_bindgen::JsCast;

    let window = web_sys::window().ok_or("No window available")?;
    let document = window.document().ok_or("No document available")?;
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::JsValue::from_str(content));

    let options = web_sys::BlobPropertyBag::new();
    options.set_type("text/csv;charset=utf-8;");
    let blob = web_sys::Blob::new_with_str_sequence_and_options(&array, &options)
        .map_err(|e| format!("{:?}", e))?;

    let url = web_sys::Url::create_object_url_with_blob(&blob).map_err(|e| format!("{:?}", e))?;
    let link = document
        .create_element("a")
        .map_err(|e| format!("{:?}", e))?
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .map_err(|_| "Failed to cast to HtmlAnchorElement")?;

    link.set_href(&url);
    link.set_download(filename);
    let _ = link.style().set_property("visibility", "hidden");

    if let Some(body) = document.body() {
        let _ = body.append_child(&link);
        link.click();
        let _ = body.remove_child(&link);
    }
    let _ = web_sys::Url::revoke_object_url(&url);
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn export_users_csv() -> Result<usize, String> {
    let window = web_sys::window().ok_or("No window available")?;
    let users = load_users_raw();
    if users.is_empty() {
        let _ = window.alert_with_message("No user data to export");
        return Ok(0);
    }

    let count = users.len();
    let csv = recommender_core::export::export_users_to_csv(&users);
    trigger_csv_download("champion_recommender_users.csv", &csv)?;
    let _ = window.alert_with_message(&format!("Successfully exported {count} users to CSV file!"));
    Ok(count)
}

#[cfg(target_arch = "wasm32")]
pub fn export_table_csv() -> Result<usize, String> {
    let users = load_users_raw();
    if users.is_empty() {
        return Ok(0);
    }
    let count = users.len();
    let csv = recommender_core::export::export_table_to_csv(&users);
    trigger_csv_download("champion_recommendations.csv", &csv)?;
    Ok(count)
}

#[cfg(target_arch = "wasm32")]
pub fn show_admin_panel() {
    if let Some(window) = web_sys::window() {
        let users = load_users_raw();
        let message = format!(
            "Enhanced Admin Panel\n\nLocal storage users: {}\n\nOptions:\n- OK: Export CSV file\n- Cancel: Close panel\n\nNote: Press Ctrl+Shift+E to export CSV.",
            users.len()
        );
        if window.confirm_with_message(&message).unwrap_or(false) {
            let _ = export_users_csv();
        }
    }
}
