const js = import("./image_black_white");

const fileUploader = document.querySelector("#uploadfile");

fileUploader.addEventListener('change', (event) => {
  const file = event.target.files[0];

  var img = new Image;

  img.onload = function() {
    js.then(js => {
      js.greet(img);
    });
  }

  img.src = URL.createObjectURL(file);
})


