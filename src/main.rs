mod util;

use serde::Serialize;
use tungstenite::{accept, Message};
use util::audio_tap;
use std::{
  net::TcpListener, 
  sync::{
    mpsc::channel,
    Arc,
    Mutex, OnceLock
  }, 
  thread::spawn
};

#[derive(Debug, Serialize, Clone, Copy)]
struct Header {
  pub channels: u16,
  pub samplerate: u32,
  pub blocksize: u32
}


fn main() -> std::io::Result<()> {
  let header: OnceLock<Header> = OnceLock::<Header>::new();
  let (tx, rx) = channel::<Vec<f32>>();
  let ws_que = Arc::new(Mutex::new(rx));


  std::thread::spawn(move || audio_tap(tx));

  let url = "127.0.0.1:8080";
  let server = TcpListener::bind(url).unwrap();
  for stream in server.incoming() {
    let inner_ws_que = ws_que.clone();
    // let inner_header = header;
    spawn(move || {
      let mut ws = accept(stream.unwrap()).unwrap();
      // if let Some(head) = inner_header.get() {
      //
      //   ws.send(Message::Text(
      //     serde_json::to_string(
      //       head
      //     ).unwrap()
      //   ));
      // }
      loop{
        let data = inner_ws_que.try_lock().unwrap().recv().unwrap(); 
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
