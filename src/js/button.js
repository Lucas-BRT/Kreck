import { handleLocalIp } from "./local_ip.js";
import {
    shutdownServer,
    launchServer,
    openQrCodeWindow,
    openConfigWindow,
} from "./tauriCommands.js";
import { getIp, getPort } from "./utils.js";

function updateButton(button, style, innerText) {
    button.className = style;
    button.innerText = innerText;
}

export function handleConnectButton(querySelector) {
    let buttonState = "unselected";
    let connectButton = document.querySelector("#connect-button");

    connectButton.addEventListener("click", () => {
        if (buttonState == "unselected") {
            updateButton(
                connectButton,
                "selected-connect-button",
                "DISCONNECT"
            );
            buttonState = "selected";
            const ip = getIp();
            const port = getPort();
            launchServer(ip, port);
            handleLocalIp();
        } else {
            updateButton(connectButton, "unselected-connect-button", "CONNECT");
            buttonState = "unselected";
            shutdownServer();
            handleLocalIp();
        }
    });

    let qrCodeButton = document.querySelector("#qr-code-button");

    qrCodeButton.addEventListener("click", async () => {
        await openQrCodeWindow();
    });

    let configButton = document.querySelector("#config-button");

    configButton.addEventListener("click", async () => {
        await openConfigWindow();
    });
}
