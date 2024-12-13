mod util;

use anyhow::Context;
use tungstenite::{accept, Message};
use util::{
  audio_tap,
  Header,
  AudioBlock
};

use serde_json::json;
use std::{
  cell::LazyCell, net::TcpListener, sync::{
    mpsc::channel,
    Arc,
    Mutex, OnceLock, LazyLock
  }, thread::spawn
};



fn main() -> std::io::Result<()> {
  let (tx_header, rx_header) = channel::<Header>();
  let (tx_audioblock, rx_audioblock) = channel::<AudioBlock>();
  let ws_que = Arc::new(Mutex::new(rx_audioblock));
  let ws_header = Arc::new(Mutex::new(rx_header));

  std::thread::spawn(move || audio_tap(tx_audioblock, tx_header));

  let url = "127.0.0.1:8080";
  let server = TcpListener::bind(url).unwrap();
  for stream in server.incoming() {
    let inner_ws_que = ws_que.clone();
    let inner_ws_header = ws_header.clone();
    spawn(move || {
      let mut ws = accept(stream.unwrap()).unwrap();
      if let Ok(head) = inner_ws_header.lock() {
        if let Ok(head) = head.recv() {
          let _ = ws.send(Message::text(
              json!({
                "channels": head.channels,
                "samplerate": head.samplerate,
                "blocksize": head.blocksize
              }
            ).to_string()));
        } else {
          println!("hello");
        }
      }
      loop{
        let AudioBlock(data) = inner_ws_que.try_lock().unwrap().recv().unwrap(); 
        unsafe {
          ws.send(
            Message::Binary( 
              std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * 4
              ).to_vec()
            )
          ).unwrap();
        }
      }
    });
  }
  
  Ok(())
}
