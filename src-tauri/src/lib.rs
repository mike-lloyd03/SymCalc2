use kalk::parser;

#[tauri::command]
fn calc(expression: &str) -> String {
    let mut parser_context = parser::Context::new();
    let result = parser::eval(&mut parser_context, &expression)
        .unwrap()
        .unwrap();
    println!("Got expression: {expression}");

    result.to_js_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![calc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
