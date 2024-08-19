use core::time;
use std::{
  sync::mpsc::Sender,
  thread
};

use anyhow::Context;

use cpal::{
  default_host,
  traits::{DeviceTrait, HostTrait, StreamTrait}, 
  StreamConfig
};


/// Captures default input and transfers to default output. 
/// Hijacks the buffer and sends it trough websockets. 
pub fn audio_tap(que: Sender<Vec<f32>>) -> anyhow::Result<()> {
  let mut keep_alive = true;
  let host = default_host();
  let src = host .default_input_device()
    .context("could not find the default output device.")?;
  let dest = host.default_output_device()
    .context("could not find the default output device.")?;
  
  let i_conf: StreamConfig = src.default_input_config()?.into(); 
  let o_conf: StreamConfig = dest.default_output_config()?.into();

  let (tx, rx) = std::sync::mpsc::channel::<f32>();
  
  let input_cb = move |data: &[f32], _: &cpal::InputCallbackInfo| {
    let _ = que.send(data.to_vec());
    for &sample in data {
      let _ = tx.send(sample);
    }
  };

  let output_cb = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
    for sample in data.iter_mut() {
      *sample = rx.try_recv().unwrap_or(0.0); 
    }
    
      // copy the audio to the queue. this is what is sent through the socket.
    // println!("{:?}", data);
  };

  let error_cb = |err: cpal::StreamError| { eprintln!("{}", err); };

  let i_stream = src.build_input_stream(
    &i_conf,
    input_cb,
    error_cb,
    None
  )?;

  let o_stream = dest.build_output_stream(
    &o_conf,
    output_cb,
    error_cb,
    None
  )?;

  i_stream.play().expect("input stream failed");
  o_stream.play().expect("output stream failed");

  let mut buf = String::new();
  loop {
    if keep_alive {
      thread::sleep(time::Duration::from_secs(1));
    } else {
      break;
    }

    let _ = std::io::stdin().read_line(&mut buf);
    if buf.starts_with('q') {
      keep_alive = false;
    }
    buf.clear();
  }

  Ok(())
}

