import { getQrCodeAsMatrix } from "./tauriCommands.js";

getQrCodeAsMatrix().then((response) => {
    console.log(response);
    const cellSize = 6;

    // Calcula o tamanho da matriz
    const matrixSize = response.length;

    // Seleciona o canvas e ajusta seu tamanho
    const canvas = document.querySelector("#qr-canvas");
    canvas.width = matrixSize * cellSize;
    canvas.height = matrixSize * cellSize;

    const ctx = canvas.getContext("2d");

    // Itera sobre a matriz e desenha os retângulos
    for (let i = 0; i < matrixSize; i++) {
        const row = response[i];
        for (let j = 0; j < row.length; j++) {
            const cellValue = row[j];
            ctx.fillStyle = cellValue ? "black" : "white";
            ctx.fillRect(j * cellSize, i * cellSize, cellSize, cellSize);
        }
    }
});
