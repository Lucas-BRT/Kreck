import { closeWindow, getConfig, setConfig } from "./tauriCommands.js";

export function handleIpInput(address) {
    let ipInputs = document.querySelectorAll(".ip-entry");

    const ips = address.split(".");

    for (let i = 0; i < ipInputs.length; i++) {
        ipInputs[i].value = ips[i];
    }

    ipInputs.forEach((input) => {
        input.addEventListener("input", () => {
            input.value = input.value.replace(/[^0-9]/g, "");

            if (input.value.length > 3) {
                input.value = input.value.slice(0, 3);
            }

            const validIpPart = /^(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)$/;

            if (!validIpPart.test(input.value)) {
                if (parseInt(input.value, 10) > 255) {
                    input.value = "255";
                }
            }
        });
    });
}

export function handlePortInput() {
    let inputField = document.querySelector("#port");

    inputField.addEventListener("input", (event) => {
        let inputValue = event.target.value;
        let regex =
            /^([1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])$/;

        inputValue = inputValue.replace(/[^0-9]/g, "");

        if (inputValue.length > 5) {
            inputValue = inputValue.slice(0, 5);
        }

        if (!regex.test(inputValue)) {
            event.target.value = inputValue.slice(0, -1);
        } else {
            event.target.value = inputValue;
        }
    });
}

function handleSaveButton() {
    const saveButton = document.querySelector("#save-button");

    saveButton.addEventListener("click", async () => {
        // get the ip address

        const [ipElement1, ipElement2, ipElement3, ipElement4] =
            document.querySelectorAll(".ip-entry");

        const ip1 = ipElement1.value == "" ? 127 : ipElement1.value;
        const ip2 = ipElement2.value == "" ? 0 : ipElement2.value;
        const ip3 = ipElement3.value == "" ? 0 : ipElement3.value;
        const ip4 = ipElement4.value == "" ? 1 : ipElement4.value;

        const address = `${ip1}.${ip2}.${ip3}.${ip4}`;

        // get port address

        const portElement = document.querySelector("#port");

        const port =
            portElement.value == "" ? 3333 : parseInt(portElement.value);

        console.log(address, port);

        // save the current address

        await setConfig(address, port);

        await closeWindow();
    });
}

let a = await getConfig();
let { address, port } = a.kenku_remote_address;

handleIpInput(address);
handlePortInput(port);
handleSaveButton();
