#[tauri::command]
pub async fn create_window(app: tauri::AppHandle, label: String, url: String) {
    let webview_window =
        tauri::WebviewWindowBuilder::new(&app, label, tauri::WebviewUrl::App(url.into()))
            .build()
            .unwrap();
}

#[tauri::command]
pub async fn reopen_window(app: tauri::AppHandle) {
    let webview_window = tauri::WebviewWindowBuilder::from_config(
        &app,
        &app.config().app.windows.get(0).unwrap().clone(),
    )
    .unwrap()
    .build()
    .unwrap();
}
