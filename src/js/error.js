import { closeWindow, openIssuesPage } from "./tauriCommands.js";

const tauri = window.__TAURI__;
const { listen } = tauri.event;
const emit = tauri.event.emit;
const invoke = tauri.core.invoke;

const errorMessage = document.getElementById("error-message");
const okButton = document.getElementById("ok-button");
const reportButton = document.getElementById("report-button");

listen("error", (event) => {
    console.log(event.payload);
    errorMessage.innerText = event.payload;
});

await emit("error-window-ready");

okButton.addEventListener("click", () => {
    closeWindow();
});

reportButton.addEventListener("click", () => {
    openIssuesPage();
    closeWindow();
});
