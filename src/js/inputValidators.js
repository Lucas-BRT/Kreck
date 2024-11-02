export function handleIpInput() {
    let ipInputs = document.querySelectorAll(".ip-entry");
    console.log(ipInputs);
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

        // Remova caracteres não numéricos
        inputValue = inputValue.replace(/[^0-9]/g, "");

        // Limite a entrada a no máximo 5 caracteres para evitar números muito longos
        if (inputValue.length > 5) {
            inputValue = inputValue.slice(0, 5);
        }

        // Valide a entrada usando o regex
        if (!regex.test(inputValue)) {
            // Se a entrada não for válida, corte o último caractere
            event.target.value = inputValue.slice(0, -1);
        } else {
            event.target.value = inputValue;
        }
    });
}

handleIpInput();
handlePortInput();
