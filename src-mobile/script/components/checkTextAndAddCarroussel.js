function getTrackElements(className) {
  const elements = document.querySelectorAll(`.${className}`);
  return elements;
}

export function checkText(className) {
  const elements = getTrackElements(className);

  elements.forEach((element) => {
    const textElement = element.textContent;

    //this switch is to verify text sizes.
    switch (true) {
      case textElement.length > 21 && textElement.length < 40:
        element.className = `carrousel-small ${className} `;
        break;
      case textElement.length > 40 && textElement.length < 90:
        element.className = `carrousel-medium ${className} `;
        break;
      case textElement.length > 90:
        element.className = `carrousel-large ${className} `;
        break;

      default:
        element.className = `${className} `;
    }
  });
}
