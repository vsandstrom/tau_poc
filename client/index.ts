// PROOF OF CONCEPT AUDIO BUFFER RECIEVER
// let ctx = new window.AudioContext();

let ws = new WebSocket("http://127.0.0.1:8080");
ws.binaryType = "arraybuffer";

ws.onmessage = (event) => {
  let block = new Float32Array(event.data);
  console.log(block);
}


