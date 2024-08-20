// PROOF OF CONCEPT AUDIO BUFFER RECIEVER
let ctx = new window.AudioContext();



new WebSocket("http://127.0.0.1:8080").onmessage = (event) => {

  console.log(JSON.parse(event.data).data);
}


