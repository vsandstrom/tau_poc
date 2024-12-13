use serde::Serialize;
use std::sync::{OnceLock, Arc};
use serde_json;
use std::{
  sync::mpsc::Sender,
  thread,
  time
};


use anyhow::Context;

use cpal::{
  default_host, 
  traits::{DeviceTrait, HostTrait, StreamTrait}, BufferSize, SampleRate, StreamConfig
};

#[derive(Debug, Serialize, Clone, Copy)]
pub struct Header {
  pub channels: u16,
  pub samplerate: u32,
  pub blocksize: u32
}

pub struct AudioBlock(pub Vec<f32>);


/// Captures default input and transfers to default output. 
/// Hijacks the buffer and sends it trough websockets. 
pub fn audio_tap(tx_audioblock: Sender<AudioBlock>, tx_header: Sender<Header>) -> anyhow::Result<()> {
  let mut keep_alive = true;
  let host = default_host();
  let src = host .default_input_device()
    .context("could not find the default output device.")?;
  let dest = host.default_output_device()
    .context("could not find the default output device.")?;
  
  let i_conf: StreamConfig = src.default_input_config()?.into(); 
  let o_conf: StreamConfig = dest.default_output_config()?.into();

  let (tx, rx) = std::sync::mpsc::channel::<f32>();

  let _h = Header{
    channels: i_conf.channels,
    samplerate: {
      let SampleRate(sr) = i_conf.sample_rate; 
      sr
    },
    blocksize: 512
    // match i_conf.buffer_size {
    //   BufferSize::Fixed(size) => size,
    //   _ => panic!("no set samplerate")
    // }
  };

  tx_header.send(_h);
  // header.set(h).expect("Header unable to be set.");
  
  let input_cb = move |data: &[f32], _: &cpal::InputCallbackInfo| {
    let _ = tx_audioblock.send(AudioBlock(data.to_vec()));
    for &sample in data {
      let _ = tx.send(sample);
    }
  };


  let output_cb = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
    for sample in data.iter_mut() {
      *sample = rx.try_recv().unwrap_or(0.0); 
    }
    
      // copy the audio to the tx_audioblockue. this is what is sent through the socket.
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

