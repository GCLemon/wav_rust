//////////////////////////////////////////////////
//
//    main.rs
//    wave_rust
//

// failure
use failure::{Error, format_err};

// rust std
use std::env::*;
use std::time::Instant;

// inner modules
pub mod wave;
pub mod sound;

fn main() -> Result<(), Error>
{
    // get commandline arguments
    let args: Vec<String> = args().collect();

    // create wav file data
    let start = Instant::now();
    let (info, data) = match wave::new(&args[1])
    {
        Ok(tuple) => tuple,
        Err(message) =>
        {
            println!("{}", message);
            return Err(format_err!("Failed to read data."));
        }
    };
    let end = start.elapsed();
    println!("{}.{:09} seconds for loading.", end.as_secs(), end.subsec_nanos());

    // play wav sound
    sound::stream_play(&info, &data)?;

    // success
    Ok(())
}
