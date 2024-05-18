export function getIp() {
  const ip_inputs = document.querySelectorAll(".ip-entry")
  let ip_array = []

  ip_inputs.forEach(ip => {
    ip_array.push(ip.value)
  })

  return ip_array.join(".")
}

export function getPort() {
  const port_input = document.getElementById("port")

  return parseInt(port_input.value)
}


