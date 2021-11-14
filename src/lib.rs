use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn fetch_data(url: String) -> Result<String, JsValue> {
    let response = reqwest::get(&url).await?;
    let json = response.text().await?;

    let window = web_sys::window().expect("no global 'window' exists");
    let document = window.document().expect("should have a document on window");
    document.body().unwrap().set_text_content(Some(&json));

    Ok(json)
}
