import { getLocalIp } from "./tauriCommands.js"

export async function handleLocalIp() {
  let ipContainer = document.getElementById("host-ip-address")

  if (ipContainer.className == "invisible-local-ip") {
    ipContainer.className = "visible-local-ip"
    ipContainer.innerHTML = `<span>access http://${await getLocalIp()}:8000 on your mobile device</span>`
  } else {
    ipContainer.className = "invisible-local-ip"
  }
}

