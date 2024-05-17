document.addEventListener("DOMContentLoaded", function () {
  document.querySelector(".play-button").addEventListener("click", () => {
    let playbackbar = document.querySelector(".playbackbar");
    playbackbar.style.display = "flex";
    console.log("Div");
  });
});
