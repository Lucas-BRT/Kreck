import { closeWindow, getQrCodeAsMatrix } from "./tauriCommands.js";

const closeButton = document.querySelector("#close-button");

closeButton.addEventListener("click", async () => {
    await closeWindow();
});

getQrCodeAsMatrix().then((response) => {
    console.log(response);
    const cellSize = 6;

    const matrixSize = response.length;

    const canvas = document.querySelector("#qr-canvas");
    canvas.width = matrixSize * cellSize;
    canvas.height = matrixSize * cellSize;

    const ctx = canvas.getContext("2d");

    for (let i = 0; i < matrixSize; i++) {
        const row = response[i];
        for (let j = 0; j < row.length; j++) {
            const cellValue = row[j];
            ctx.fillStyle = cellValue ? "black" : "white";
            ctx.fillRect(j * cellSize, i * cellSize, cellSize, cellSize);
        }
    }
});
