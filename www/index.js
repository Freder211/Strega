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
    let extention = get_file_name_extention(input_file.name);
    reader.readAsArrayBuffer(input_file);
   
    reader.addEventListener('load', function (e) {
        return cb(e, extention);
    });
  
}

function encodeFile(event, extention) {
  let bytes_buffer = event.target.result;
  const bytes = new Int8Array(bytes_buffer);
  const encoded_bytes = strega.encode_file(bytes, getInputText(), extention);
  downloadFile([encoded_bytes], extention);
}

function decodeFile(event, extention) {
    let bytes_buffer = event.target.result;
    const bytes = new Int8Array(bytes_buffer);
    let text = strega.decode_file(bytes, extention);
    showOutputText(text);
  
}

function downloadFile(bytes, extention) {
    var a = window.document.createElement('a');
    a.href = window.URL.createObjectURL(new Blob(bytes));
    a.download = "out." + extention;
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

function get_file_name_extention(file_name) {
  return file_name.split(".").pop();
}
