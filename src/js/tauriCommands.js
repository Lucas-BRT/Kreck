const tauri = window.__TAURI__;
const invoke = tauri.core.invoke;

function shutdownServer() {
    invoke("request_server_shutdown", {});
}

async function launchServer(ip, port) {
    return invoke("launch_server", { ip, port });
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

async function openErrorWindow() {
    await invoke("open_error_window");
}

async function getQrCodeAsMatrix() {
    return await invoke("get_qr_code_as_matrix");
}

async function getConfig() {
    return await invoke("get_config");
}

async function setConfig(address, port) {
    await invoke("set_config", { address, port });
}

async function closeWindow() {
    await invoke("close_window");
}

async function emitError(message) {
    await invoke("emit_error", { message });
}

function openIssuesPage() {
    invoke("open_issues_page");
}

export {
    shutdownServer,
    launchServer,
    getLocalIp,
    openQrCodeWindow,
    openConfigWindow,
    openErrorWindow,
    getQrCodeAsMatrix,
    getConfig,
    setConfig,
    closeWindow,
    emitError,
    openIssuesPage,
};
