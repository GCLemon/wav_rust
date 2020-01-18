//////////////////////////////////////////////////
//
//    sound.rs
//    波形データを受け取って再生するサンプル
//

extern crate alto;
extern crate failure;

// alto
use alto::*;

// failure
use failure::Error;

// rust std
use std::cmp::min;

// inner modules
use crate::wave::*;

pub fn stream_play(info: &WaveInformation, data: &WaveBuffer) -> Result<(), Error>
{
    let alto = Alto::load_default().unwrap();
    let device = alto.open(None).unwrap();
    let context = device.new_context(None).unwrap();

    let mut source = context.new_streaming_source().unwrap();
    let mut que_count = 0;

    for i in 0..
    {
        let rate = info.sampling_rate;
        let curr = ((rate * 4) * i) as usize;
        let buffer = match data
        {
            WaveBuffer::U8Mono(data) =>
            {
                if ((rate * 4) * i) as usize >= data.len() { break; }
                let next = min(((rate * 4) * (i + 1)) as usize, data.len());
                context.new_buffer(&data[curr..next], rate as i32).unwrap()
            },
            WaveBuffer::I16Mono(data) =>
            {
                if ((rate * 4) * i) as usize >= data.len() { break; }
                let next = min(((rate * 4) * (i + 1)) as usize, data.len());
                context.new_buffer(&data[curr..next], rate as i32).unwrap()
            },
            WaveBuffer::U8Stereo(data) =>
            {
                if ((rate * 4) * i) as usize >= data.len() { break; }
                let next = min(((rate * 4) * (i + 1)) as usize, data.len());
                context.new_buffer(&data[curr..next], rate as i32).unwrap()
            },
            WaveBuffer::I16Stereo(data) =>
            {
                if ((rate * 4) * i) as usize >= data.len() { break; }
                let next = min(((rate * 4) * (i + 1)) as usize, data.len());
                context.new_buffer(&data[curr..next], rate as i32).unwrap()
            },
        };
        source.queue_buffer(buffer).unwrap();
        que_count += 1;
    }

    source.play();
    while source.buffers_processed() < que_count { }


    Ok(())
}