export function putFade(classFade, carouselName, duration) {
  const carousels = document.querySelectorAll(`.${carouselName}`);
  const animationDuration = duration;

  let animationStartTime = null;

  // Function to toggle the "classFade" class based on animation progress
  const toggleTrackNameFade = (progress) => {
    carousels.forEach((carousel) => {
      if (progress >= 0 && progress <= 20) {
        carousel.parentElement.classList.remove(`${classFade}`);
      } else {
        carousel.parentElement.classList.add(`${classFade}`);
      }
    });
  };

  // Function to update the carousel based on current time and animation duration
  const updateCarousel = () => {
    const currentTime = Date.now();
    const elapsedTime = (currentTime - animationStartTime) % animationDuration;
    const animationProgress = (elapsedTime / animationDuration) * 100;

    toggleTrackNameFade(animationProgress);
    requestAnimationFrame(updateCarousel);
  };

  // Initialize animation start time on animation iteration
  carousels.forEach((carousel) => {
    carousel.addEventListener("animationiteration", () => {
      animationStartTime = Date.now();
    });
  });

  // Start monitoring the carousel animation
  animationStartTime = Date.now();
  requestAnimationFrame(updateCarousel);
}
