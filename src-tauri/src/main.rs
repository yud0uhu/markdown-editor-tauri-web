#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(serde::Serialize)]
struct ApiResponse {
    img: String,
}

fn encode_to_base64(input: &str) -> String {
    base64::encode(input)
}


#[tauri::command]
fn generate_mermaid_img(code: String) -> String {
    let encoded_code = encode_to_base64(&code);

    let response = ApiResponse {
        img: format!("https://mermaid.ink/img/{}", encoded_code),
    };

    format!("{}", response.img)

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_mermaid_img])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
