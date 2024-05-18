// https://localhost:8000/tracks

fetch("https://jsonplaceholder.typicode.com/posts/", {
  method: "GET",
  headers: { "Content-Type": "application/json" },
})
  .then((res) => res.json())
  .then((data) => {
    console.log(data);
    const content_music = document.querySelector(".content");
    data.forEach((track) => {
      console.log(track);
      let music = document.createElement("div");
      music.innerHTML = `
    <div class="music-informations ">
    <div class="music-name">${track.title.substring(0, 7)}</div> 
    <div class="music-time">${track.body.substring(0, 7)}</div>
    </div>
    <div class="music-player"><div class="play-button"><div class="player-button-triangle">
    </div>
    </div>
    </div>`;
      music.className = "container-music";
      content_music.appendChild(music);
    });
  })
  .catch((error) => {
    console.error("Erro:", error);
  });

// let mockActived = {
//   className: "container-music",
//   innerHTML:
//     '<div class="music-informations "><div class="music-name">Music Name</div> <div class="music-time">3:06</div></div><div class="music-player"><div class="play-button"><div class="player-button-triangle"></div></div></div>',
// };

// let contentMusic = document.querySelector(".content");

// let music = document.createElement("div");
// music.className = mockActived.className;
// music.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(music);

// let music2 = document.createElement("div");
// music2.className = mockActived.className;
// music2.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(music2);

// let music3 = document.createElement("div");
// music3.className = mockActived.className;
// music3.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(music3);

// let music4 = document.createElement("div");
// music4.className = mockActived.className;
// music4.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(music4);

// let music5 = document.createElement("div");
// music5.className = mockActived.className;
// music5.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(music5);

// let music6 = document.createElement("div");
// music6.className = mockActived.className;
// music6.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(music6);

// let music7 = document.createElement("div");
// music7.className = mockActived.className;
// music7.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(music7);

// let musci8 = document.createElement("div");
// musci8.className = mockActived.className;
// musci8.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(musci8);

// let music9 = document.createElement("div");
// music9.className = mockActived.className;
// music9.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(music9);

// let music10 = document.createElement("div");
// music10.className = mockActived.className;
// music10.innerHTML = mockActived.innerHTML;
// contentMusic.appendChild(music10);
