// PROOF OF CONCEPT AUDIO BUFFER RECIEVER

new WebSocket("http://127.0.0.1:8080").onmessage = (event) => {
  console.log(typeof(JSON.parse(event.data).data[0]));
}


