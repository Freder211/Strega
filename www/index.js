import * as strega from "strega";

// strega.test_gatto();

/* var a = document.createElement("a");
* a.href = window.URL.createObjectURL(new Blob(["CONTENT"], {type: "text/plain"}));
* a.download = "demo.txt";
* a.click(); */
function readFile(event) {
  let array_buffer = event.target.result;
  strega.encode_file(array_buffer);
}

document.getElementById("confirm").addEventListener("click", () => {
    let input_file = document.getElementById("input-file").files[0];
    let reader = new FileReader();
    let bytes = reader.readAsArrayBuffer(input_file);
    reader.addEventListener('load', readFile);
    console.log(bytes);
})


// strega.encode_file([99, 105, 97, 111]);
