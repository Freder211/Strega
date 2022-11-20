import * as strega from "strega";


/* var a = document.createElement("a");
* a.href = window.URL.createObjectURL(new Blob(["CONTENT"], {type: "text/plain"}));
* a.download = "demo.txt";
* a.click(); */

document.getElementById("confirm-encode").addEventListener("click", () => {
    readInputFile(encodeFile);
})

document.getElementById("confirm-decode").addEventListener("click", () => {
    readInputFile(decodeFile);
})

function readInputFile(cb) {
    let input_file = document.getElementById("input-file").files[0];
    let reader = new FileReader();
    reader.readAsArrayBuffer(input_file);
    reader.addEventListener('load', cb);
  
}

function encodeFile(event) {
  let bytes_buffer = event.target.result;

  const bytes = new Int8Array(bytes_buffer);
    const encoded_bytes = strega.encode_file(bytes, getInputText());
  downloadFile([encoded_bytes]);
}

function decodeFile(event) {
    let bytes_buffer = event.target.result;
    const bytes = new Int8Array(bytes_buffer);
    let text = strega.decode_file(bytes);
    showOutputText(text);
  
}

function downloadFile(bytes) {
    var a = window.document.createElement('a');
    a.href = window.URL.createObjectURL(new Blob(bytes));
    a.download = "out.png";
    document.body.appendChild(a)
    a.click();
    document.body.removeChild(a)
}

function getInputText() {
    const text = document.getElementById("input-text").value;
  return text;
}

function showOutputText(text) {
    document.getElementById("output-text").innerHTML = text;
}
