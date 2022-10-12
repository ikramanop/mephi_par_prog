use std::fs::File;
use std::path::Path;
use std::sync::mpsc::Sender;
use wav::BitDepth;

pub struct ThreadParamsF32 {
    pub data: Vec<f32>,
    pub tx: Sender<f32>,
    pub i: usize,
}

pub fn process_wav_file(path: String) -> anyhow::Result<BitDepth> {
    let mut file = File::open(Path::new(&path))?;

    let (_, wav_data) = wav::read(&mut file)?;

    Ok(wav_data)
}

pub fn convert_u8(data: &[u8]) -> Vec<f32> {
    let mut result = Vec::<f32>::new();

    for x in data {
        result.push(*x as f32)
    }

    result
}

pub fn convert_i16(data: &[i16]) -> Vec<f32> {
    let mut result = Vec::<f32>::new();

    for x in data {
        result.push(*x as f32)
    }

    result
}

pub fn convert_i32(data: &[i32]) -> Vec<f32> {
    let mut result = Vec::<f32>::new();

    for x in data {
        result.push(*x as f32)
    }

    result
}
