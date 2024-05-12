export function handleIpInput(querySelector) {
  let ipInputs = document.querySelectorAll(querySelector)

  ipInputs.forEach((input) => {
    input.addEventListener('input', (event) => {
      let inputValue = event.target.value;
      let regex = /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;

      if (!regex.test(inputValue)) {
        event.target.value = inputValue.slice(0, -1);

        if (event.target.value == '') {
          event.target.value = 0;
        }
      }
    })
  })
}

export function handlePortInput(querySelector) {
  let inputFields = document.querySelectorAll(querySelector);

  inputFields.forEach((input) => {
    input.addEventListener('input', (event) => {
      let inputValue = event.target.value;
      let regex = /^([1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])$/;

      if (!regex.test(inputValue)) {
        event.target.value = inputValue.slice(0, -1);
      }
    });
  });
}






