// PROOF OF CONCEPT AUDIO BUFFER RECIEVER
// let ctx = new window.AudioContext();
var header;
var c = 0;
var ws = new WebSocket("http://127.0.0.1:8080");
ws.binaryType = "arraybuffer";
ws.onopen = function (header) {
    console.log("header received");
    console.log(header);
};
// @ts-expect-error
var ctx = new (window.AudioContext || window.webkitAudioContext);
// let ctx = new AudioContext();
var src = ctx.createBufferSource();
// console.log(src.buffer?.length);
// console.log(src.buffer?.sampleRate)
/// SET THESE VALUES FROM CONNECTION HEADER
ws.onmessage = function (event) {
    if (event.data instanceof ArrayBuffer) {
        var block = new Float32Array(event.data);
        console.log("block", block);
    }
    else {
        header = JSON.parse(event.data);
        console.log(header);
        // let {ch, sr, bs} = head;
        // header.channels = ch;
        // header.samplerate = sr;
        // header.blocksize = bs;
        // console.log(
        //   "channels: ", header.channels, 
        //   "\nsamplerate: ", header.samplerate, 
        //   "\nblocksize: ", header.blocksize
        // );
    }
    c++;
};
