import {
    shutdownServer,
    launchServer,
    openQrCodeWindow,
    openConfigWindow,
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

export function handleIpInput(querySelector) {
    let ipInputs = document.querySelectorAll(querySelector);

    ipInputs.forEach((input) => {
        input.addEventListener("input", (event) => {
            let inputValue = event.target.value;
            let regex = /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

            if (!regex.test(inputValue)) {
                event.target.value = inputValue.slice(0, -1);

                if (event.target.value == "") {
                    event.target.value = 0;
                }
            }
        });
    });
}

export function handlePortInput(querySelector) {
    let inputFields = document.querySelectorAll(querySelector);

    inputFields.forEach((input) => {
        input.addEventListener("input", (event) => {
            let inputValue = event.target.value;
            let regex =
                /^([1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])$/;

            if (!regex.test(inputValue)) {
                event.target.value = inputValue.slice(0, -1);
            }
        });
    });
}
