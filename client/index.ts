// PROOF OF CONCEPT AUDIO BUFFER RECIEVER
// let ctx = new window.AudioContext();

let header;
let c = 0;

let ws = new WebSocket("http://127.0.0.1:8080");
ws.binaryType = "arraybuffer";

ws.onopen = (header) => {
  console.log("header received");
  console.log(header);
}

// @ts-expect-error
let ctx = new (window.AudioContext || window.webkitAudioContext);
// let ctx = new AudioContext();
let src = ctx.createBufferSource();
// console.log(src.buffer?.length);
// console.log(src.buffer?.sampleRate)

/// SET THESE VALUES FROM CONNECTION HEADER


ws.onmessage = (event) => {
  if (event.data instanceof ArrayBuffer) {
    let block = new Float32Array(event.data);
    // console.log("block", block);
  } else {
    header = JSON.parse(event.data);
    console.log(header);
  }
  c++;

}
