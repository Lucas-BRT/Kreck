import {
    shutdownServer,
    launchServer,
    openQrCodeWindow,
    openConfigWindow,
    getConfig,
} from "./tauriCommands.js";

function updateButton(button, style, innerText) {
    button.className = style;
    button.innerText = innerText;
}

export function handleConnectButton() {
    let buttonState = "unselected";
    const connectButton = document.querySelector("#connect-button");

    connectButton.addEventListener("click", () => {
        if (buttonState == "unselected") {
            updateButton(
                connectButton,
                "selected-connect-button",
                "DISCONNECT",
            );
            const {
                kenku_remote_address: { address, port },
            } = getConfig();

            buttonState = "selected";
            launchServer(address, port);
        } else {
            updateButton(connectButton, "unselected-connect-button", "CONNECT");
            buttonState = "unselected";
            shutdownServer();
        }
    });
}

export function handleQrCodeButton() {
    const qrCodeButton = document.querySelector("#qr-code-button");

    qrCodeButton.addEventListener("click", async () => {
        await openQrCodeWindow();
    });
}

export function handleConfigButton() {
    const configButton = document.querySelector("#config-button");

    configButton.addEventListener("click", async () => {
        await openConfigWindow();
    });
}
