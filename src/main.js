const tauri  = window.__TAURI__;
const core = tauri.core;
const invoke = core.invoke;

let butao = document.getElementById("butao")


butao.addEventListener("click", async () => {
  
  await invoke("hello_world", {});

  if (butao.id == "butao-selecionado") {
    butao.removeAttribute("id")
    butao.id = "butao"
    butao.innerText = "Connect" 
  } else {
    butao.removeAttribute("id")
    butao.id = "butao-selecionado"
    butao.innerText = "Desconnect"
  }
  


  console.log("macacoi")
  
})


