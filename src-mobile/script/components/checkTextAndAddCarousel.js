import { addElementClass } from "./subcomponents/addElementClass.js";
import { putFade } from "./subcomponents/putFade.js";

function removeParentClass(element) {
  element.parentElement.classList.remove("no-fade");
}

export function checkTextAndAddCarousel(where) {
  const element = document.querySelectorAll(`.${where}-track-name`);
  const classFade = `${where}-name-fade`;

  element.forEach((element) => {
    const textElement = element.textContent;

    if (textElement.length > 21 && textElement.length <= 40) {
      addElementClass(element, "small", `${where}`);
      removeParentClass(element);
      putFade(classFade, `carousel-small-${where}`, 20000);
    } else if (textElement.length > 40 && textElement.length <= 90) {
      addElementClass(element, "medium", `${where}`);
      removeParentClass(element);
      putFade(classFade, `carousel-medium-${where}`, 25000);
    } else if (textElement.length > 90) {
      addElementClass(element, "large", `${where}`);
      removeParentClass(element);
      putFade(classFade, `carousel-large-${where}`, 30000);
    } else {
      element.className = `${where}-track-name `;
      element.parentElement.classList.add("no-fade");
    }
  });
}
