const tauri = window.__TAURI__;
const invoke = tauri.core.invoke;

export function shutdownServer() {
  invoke("request_server_shutdown", {});
}

export function launchServer() {
  invoke("launch_server", {});
}
