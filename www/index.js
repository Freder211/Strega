import * as strega from "strega";

/**
 * @callback fileComputation
 * @param {ArrayBuffer} bytes_buffer
 * @param {string} extention
 */

/**
 * Reads file from a file input and calls cb
 * passing the bytes as an array buffer and the extetion.
 * @param {fileComputation} cb
*/
function execOnFileReading(cb) {
    let input_file = document.getElementById("input-file").files[0];
    let reader = new FileReader();
    let extention = get_file_name_extention(input_file.name);
    reader.readAsArrayBuffer(input_file);

    reader.addEventListener('load', function (e) {
        let bytes_buffer = e.target.result;
        return cb(bytes_buffer, extention);
    });
}

/**
 * Encodes bytes with text provided by function 'getInputText'.
 * The encoded bytes are exported inside a file which is downloaded withe 'downloadFile'.
 * @param {ArrayBuffer} bytes_buffer
 * @param {string} extention
*/
function encodeFile(bytes_buffer, extention) {
    const bytes = new Int8Array(bytes_buffer);
    const encoded_bytes = strega.encode_file(bytes, getInputText(), extention);
    downloadFile([encoded_bytes], extention);
}

/**
 * Reads an encoded file and displays the text with 'showOutputText'.
 * @param {ArrayBuffer} bytes_buffer
 * @param {string} extention
*/
function decodeFile(bytes_buffer, extention) {
    const bytes = new Int8Array(bytes_buffer);
    let text = strega.decode_file(bytes, extention);
    showOutputText(text);

}

/**
 * Exports bytes to a file which is then downloaded.
 * @param {Int8Array} bytes
 * @param {string} extention
*/
function downloadFile(bytes, extention) {
    var a = window.document.createElement('a');
    a.href = window.URL.createObjectURL(new Blob(bytes));
    a.download = "out." + extention;
    document.body.appendChild(a)
    a.click();
    document.body.removeChild(a)
}

/** Reads the text from the user input */
function getInputText() {
    const text = document.getElementById("input-text").value;
    return text;
}

/**
 * Displays the text to the user
 * @param {string} text
 */
function showOutputText(text) {
    document.getElementById("output-text").innerHTML = text;
}

/**
 * Reads the extention from a file name
 * @param {string} file_name
 */
function get_file_name_extention(file_name) {
    return file_name.split(".").pop();
}

document.getElementById("confirm-encode").addEventListener("click", () => {
    execOnFileReading(encodeFile);
})

document.getElementById("confirm-decode").addEventListener("click", () => {
    execOnFileReading(decodeFile);
})
