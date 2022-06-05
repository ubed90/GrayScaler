async function init() {
  let rustApp = null;

  try {
    rustApp = await import("../pkg");
  } catch (e) {
    console.error(e);
    return;
  }

  console.log(rustApp);

  const input = document.getElementById("upload");
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    // const base64 = fileReader.result;
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );
    // console.log(input.files[0]);
    // console.log(base64);

    // rustApp.gray_scale(base64);
    let imgDataUrl = rustApp.gray_scale(base64);

    document.getElementById("new-img").setAttribute("src", imgDataUrl);
  };

  input.addEventListener("change", (e) => {
    fileReader.readAsDataURL(e.target.files[0]);
  });
}

init();
