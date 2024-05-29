import { getLocalIp } from "./tauriCommands.js"

export async function handleLocalIp() {
  let ipContainer = document.getElementById("host-ip-address")

  if (ipContainer.className == "invisible-local-ip") {
    ipContainer.className = "visible-local-ip"
    ipContainer.innerHTML = `<p>access 
      <span class="local-address">
        http://${await getLocalIp()}:8000
      </span> 
      on your mobile device </p>`
  } else {
    ipContainer.className = "invisible-local-ip"
  }
}

