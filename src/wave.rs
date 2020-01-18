//////////////////////////////////////////////////
//
//    wave.rs
//    .wavファイルを読み込むサンプル
//

extern crate byteorder;

// alto
use alto::{Mono, Stereo};

// byteorder
use byteorder::{LittleEndian, ReadBytesExt};

// failure
use failure::{Error, format_err};

// rust std
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufReader};

pub struct WaveInformation
{
    pub file_size: u32,
    pub pcm_format: u16,
    pub channels: u16,
    pub sampling_rate: u32,
    pub byte_per_sec: u32,
    pub block_align: u16,
    pub bit_per_sample: u16,
}

pub enum WaveBuffer
{
    U8Mono(Vec<Mono<u8>>),
    I16Mono(Vec<Mono<i16>>),
    U8Stereo(Vec<Stereo<u8>>),
    I16Stereo(Vec<Stereo<i16>>),
}

pub fn new(file_path: &String) -> Result<(WaveInformation, WaveBuffer), Error>
{
    // create wave reader
    let mut reader = BufReader::new(File::open(file_path)?);

    // create variable to put collected data from the .wav file
    let mut signature = [0u8; 4];

    // create wave information structure
    let mut info = WaveInformation
    {
        file_size: 0,
        pcm_format: 0,
        channels: 0,
        sampling_rate: 0,
        byte_per_sec: 0,
        block_align: 0,
        bit_per_sample: 0,
    };

    // read riff, size, wave
    reader.read_exact(&mut signature)?;
    if signature != [ 0x52, 0x49, 0x46, 0x46 ] { return Err(format_err!("This is not a wave file.")); }

    info.file_size = reader.read_u32::<LittleEndian>()? + 8;
    
    reader.read_exact(&mut signature)?;
    if signature != [ 0x57, 0x41, 0x56, 0x45 ] { return Err(format_err!("This file does not have a WAVE header.")); }

    // read chunks
    loop
    {
        reader.read_exact(&mut signature)?;
        match signature
        {
            // "fmt "
            [ 0x66, 0x6D, 0x74, 0x20 ] =>
            {
                let chunk_size = reader.read_u32::<LittleEndian>()?;

                info.pcm_format = reader.read_u16::<LittleEndian>()?;
                info.channels = reader.read_u16::<LittleEndian>()?;
                info.sampling_rate = reader.read_u32::<LittleEndian>()?;
                info.byte_per_sec = reader.read_u32::<LittleEndian>()?;
                info.block_align = reader.read_u16::<LittleEndian>()?;
                info.bit_per_sample = reader.read_u16::<LittleEndian>()?;

                reader.seek(SeekFrom::Current((chunk_size - 16) as i64))?;
            },

            // "data"
            [ 0x64, 0x61, 0x74, 0x61 ] =>
            {
                let chunk_size = reader.read_u32::<LittleEndian>()?;

                match (info.channels, info.bit_per_sample)
                {
                    (1, 8) =>
                    {
                        let mut buffer = vec![Mono{ center: 0 }; chunk_size as usize];
                        for i in 0..chunk_size
                        {
                            buffer[i as usize] = Mono
                            {
                                center: reader.read_u8()?
                            };
                        }
                        let buffer = WaveBuffer::U8Mono(buffer);
                        return Ok((info, buffer));
                    }
                    (2, 8) =>
                    {
                        let mut buffer = vec![Stereo{ left: 0, right: 0 }; (chunk_size / 2) as usize];
                        for i in 0..(chunk_size / 2)
                        {
                            buffer[i as usize] = Stereo
                            {
                                left: reader.read_u8()?,
                                right: reader.read_u8()?
                            };
                        }
                        let buffer = WaveBuffer::U8Stereo(buffer);
                        return Ok((info, buffer));
                    }
                    (1, 16) =>
                    {
                        let mut buffer = vec![Mono{ center: 0 }; (chunk_size / 2) as usize];
                        for i in 0..(chunk_size / 2)
                        {
                            buffer[i as usize] = Mono
                            {
                                center: reader.read_i16::<LittleEndian>()?
                            };
                        }
                        let buffer = WaveBuffer::I16Mono(buffer);
                        return Ok((info, buffer));
                    }
                    (2, 16) =>
                    {
                        let mut buffer = vec![Stereo{ left: 0, right: 0 }; (chunk_size / 4) as usize];
                        for i in 0..(chunk_size / 4)
                        {
                            buffer[i as usize] = Stereo
                            {
                                left: reader.read_i16::<LittleEndian>()?,
                                right: reader.read_i16::<LittleEndian>()?
                            };
                        }
                        let buffer = WaveBuffer::I16Stereo(buffer);
                        return Ok((info, buffer));
                    }
                    (_, _) => return Err(format_err!("Invalid format."))
                };
            },

            // others
            _ =>
            {
                let chunk_size = reader.read_u32::<LittleEndian>()?;
                reader.seek(SeekFrom::Current(chunk_size as i64))?;
            },
        }
    }
}