use recommender_core::contracts::StoredUserRecord;
use serde_json::Value;

pub const USERS_KEY: &str = "championRecommenderUsers";
pub const RECENT_KEY: &str = "recentChampions";

#[cfg(target_arch = "wasm32")]
pub fn get_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

#[cfg(target_arch = "wasm32")]
pub fn get_local_storage_item(key: &str) -> Option<String> {
    get_storage().and_then(|s| s.get_item(key).ok().flatten())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_local_storage_item(_key: &str) -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn set_local_storage_item(key: &str, value: &str) {
    if let Some(s) = get_storage() {
        let _ = s.set_item(key, value);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_local_storage_item(_key: &str, _value: &str) {}

#[cfg(target_arch = "wasm32")]
pub fn remove_local_storage_item(key: &str) {
    if let Some(s) = get_storage() {
        let _ = s.remove_item(key);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn remove_local_storage_item(_key: &str) {}

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
#[allow(dead_code)]
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
    let mut users = load_users_raw();
    if users.is_empty() {
        let current_user = supabase_get_current_user();
        let name = current_user
            .as_ref()
            .map(|u| u.full_name.clone())
            .unwrap_or_else(|| "Summoner".into());
        let email = current_user
            .as_ref()
            .map(|u| u.email.clone())
            .unwrap_or_else(|| "summoner@example.com".into());
        users.push(serde_json::json!({
            "fullName": name,
            "email": email,
            "phone": "Not provided",
            "age": "25",
            "gender": "Not specified",
            "experience": "Advanced",
            "registrationDate": js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default(),
            "sessionId": "session_cloud",
        }));
    }

    let count = users.len();
    let csv = recommender_core::export::export_users_to_csv(&users);
    trigger_csv_download("champion_recommender_users.csv", &csv)?;
    Ok(count)
}

#[cfg(target_arch = "wasm32")]
pub fn export_table_csv() -> Result<usize, String> {
    let mut users = load_users_raw();
    if users.is_empty() {
        let current_user = supabase_get_current_user();
        let name = current_user
            .as_ref()
            .map(|u| u.full_name.clone())
            .unwrap_or_else(|| "Summoner".into());
        let email = current_user
            .as_ref()
            .map(|u| u.email.clone())
            .unwrap_or_else(|| "summoner@example.com".into());
        let recent = load_recent_champions();
        let top_champ = recent.first().cloned().unwrap_or_else(|| "Xayah".into());
        users.push(serde_json::json!({
            "fullName": name,
            "email": email,
            "phone": "Not provided",
            "age": "25",
            "gender": "Not specified",
            "experience": "Advanced",
            "recommendedChampion": top_champ,
            "registrationDate": js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default(),
            "sessionId": "session_cloud",
        }));
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

#[cfg(target_arch = "wasm32")]
pub fn supabase_sign_in_google() {
    let _ = js_sys::eval("window.supabaseAuth && window.supabaseAuth.signInWithGoogle()");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn supabase_sign_in_google() {}

#[cfg(target_arch = "wasm32")]
pub fn supabase_sign_out() {
    let _ = js_sys::eval("window.supabaseAuth && window.supabaseAuth.signOut()");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn supabase_sign_out() {}

#[cfg(target_arch = "wasm32")]
pub fn supabase_enable_demo_mode() {
    let _ = js_sys::eval("window.supabaseAuth && window.supabaseAuth.enableDemoMode()");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn supabase_enable_demo_mode() {}

#[cfg(target_arch = "wasm32")]
pub fn supabase_get_current_user() -> Option<crate::state::AuthUser> {
    let result =
        js_sys::eval("window.supabaseAuth ? window.supabaseAuth.getCurrentUserJson() : ''").ok()?;
    let json_str = result.as_string()?;
    if json_str.trim().is_empty() {
        return None;
    }
    serde_json::from_str(&json_str).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn supabase_get_current_user() -> Option<crate::state::AuthUser> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn supabase_save_recommendation(rec: &recommender_core::contracts::RecommendationResult) {
    if let Ok(json_str) = serde_json::to_string(rec) {
        let code = format!(
            "window.supabaseAuth && window.supabaseAuth.saveRecommendation({})",
            serde_json::to_string(&json_str).unwrap_or_default()
        );
        let _ = js_sys::eval(&code);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn supabase_save_recommendation(_rec: &recommender_core::contracts::RecommendationResult) {}

#[cfg(target_arch = "wasm32")]
pub fn supabase_update_riot_profile(riot_id: &str, riot_region: &str) {
    let code = format!(
        "window.supabaseAuth && window.supabaseAuth.updateRiotProfile({}, {})",
        serde_json::to_string(riot_id).unwrap_or_default(),
        serde_json::to_string(riot_region).unwrap_or_default()
    );
    let _ = js_sys::eval(&code);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn supabase_update_riot_profile(_riot_id: &str, _riot_region: &str) {}

#[cfg(target_arch = "wasm32")]
pub fn supabase_fetch_riot_telemetry(riot_id: &str, riot_region: &str) {
    let code = format!(
        "window.supabaseAuth && window.supabaseAuth.fetchRiotTelemetry({}, {})",
        serde_json::to_string(riot_id).unwrap_or_default(),
        serde_json::to_string(riot_region).unwrap_or_default()
    );
    let _ = js_sys::eval(&code);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn supabase_fetch_riot_telemetry(_riot_id: &str, _riot_region: &str) {}

#[cfg(target_arch = "wasm32")]
pub fn load_cached_riot_matches() -> Option<Vec<recommender_core::player_analytics::AdoptedMatch>> {
    let storage = get_storage()?;
    let json_str = storage.get_item("lol_riot_telemetry").ok()??;
    let val: serde_json::Value = serde_json::from_str(&json_str).ok()?;
    let matches_val = val.get("matches")?;
    serde_json::from_value(matches_val.clone()).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_cached_riot_matches() -> Option<Vec<recommender_core::player_analytics::AdoptedMatch>> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn load_cached_riot_data_source() -> Option<String> {
    let storage = get_storage()?;
    let json_str = storage.get_item("lol_riot_telemetry").ok()??;
    let val: serde_json::Value = serde_json::from_str(&json_str).ok()?;
    val.get("data_source")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_cached_riot_data_source() -> Option<String> {
    None
}
