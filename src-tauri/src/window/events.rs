use tauri::{AppHandle, Emitter, Listener};

#[tauri::command]
pub async fn emit_error(handler: AppHandle, message: String) {
    let inner_handler = handler.clone();

    handler.listen("error-window-ready", move |_| {
        inner_handler
            .emit("error", &message)
            .expect("failed to emit error event");
    });
}
