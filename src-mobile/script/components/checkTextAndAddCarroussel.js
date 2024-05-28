function getTrackElements(className) {
  const elements = document.querySelectorAll(`.${className}`);
  return elements;
}

export function checkText(className) {
  const elements = getTrackElements(className);

  elements.forEach((element) => {
    const textElement = element.textContent;

    if (textElement.length > 21) {
      element.className = `carrousel ${className} `;
    } else {
      element.className = `${className} `;
    }
  });
}
