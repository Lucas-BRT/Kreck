import { shutdownServer, launchServer } from './tauriCommands.js'
import { getIp, getPort } from './utils.js'

function updateButton(button, style, innerText) {
  button.className = style
  button.innerText = innerText
}

export function handleConnectButton(querySelector) {
  let buttonState = "unselected"
  let connectButton = document.querySelector(querySelector)

  connectButton.addEventListener("click", () => {
    if (buttonState == "unselected") {
      updateButton(connectButton, "connect-button selected-connect-button", "Desconnect")
      buttonState = "selected"
      const ip = getIp()
      const port = getPort()
      launchServer(ip, port)
    } else {
      updateButton(connectButton, 'connect-button', "Connect")
      buttonState = "unselected"
      shutdownServer()
    }
  })
}


