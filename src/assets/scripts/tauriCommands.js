const tauri = window.__TAURI__;
const invoke = tauri.core.invoke;

export function shutdownServer() {
  invoke("request_server_shutdown", {});
}

export function launchServer(ip, port) {
  invoke("launch_server", { ip, port });
}

export async function getLocalIp() {
  return await invoke("get_host_local_ip").then(ip => ip)
}
