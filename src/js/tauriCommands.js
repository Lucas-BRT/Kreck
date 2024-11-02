const tauri = window.__TAURI__;
const invoke = tauri.core.invoke;

function shutdownServer() {
    invoke("request_server_shutdown", {});
}

function launchServer(ip, port) {
    invoke("launch_server", { ip, port });
}

async function getLocalIp() {
    return await invoke("get_host_local_address").then((ip) => ip);
}

async function openQrCodeWindow() {
    await invoke("open_qr_code_window");
}

async function openConfigWindow() {
    await invoke("open_config_window");
}

async function getQrCodeAsMatrix() {
    return await invoke("get_qr_code_as_matrix");
}

export {
    shutdownServer,
    launchServer,
    getLocalIp,
    openQrCodeWindow,
    openConfigWindow,
    getQrCodeAsMatrix,
};
