use std::fs::File;
use std::path::Path;
use anyhow::anyhow;
use wav::BitDepth;

const PIVOT: i32 = 1600;

fn main() -> anyhow::Result<()> {
    let file_path = std::env::var("WAV_FILE_PATH")?;

    let mut file = File::open(Path::new(&file_path))?;

    let (wav_header, wav_data) = wav::read(&mut file)?;

    println!("{:?}", wav_header);

    match wav_data {
        BitDepth::Eight(data) => {
            println!("Numbers bigger than {}: {}, overall {}", PIVOT, count_diff(&data, PIVOT as u8), data.len())
        }
        BitDepth::Sixteen(data) => {
            println!("Numbers bigger than {}: {}, overall {}", PIVOT, count_diff(&data, PIVOT as i16), data.len())
        }
        BitDepth::TwentyFour(data) => {
            println!("Numbers bigger than {}: {}, overall {}", PIVOT, count_diff(&data, PIVOT as i32), data.len())
        }
        BitDepth::ThirtyTwoFloat(data) => {
            println!("Numbers bigger than {}: {}, overall {}", PIVOT, count_diff(&data, PIVOT as f32), data.len())
        }
        BitDepth::Empty => {
            return Err(anyhow!("Empty wav file"));
        }
    }

    Ok(())
}

fn count_diff<T: PartialOrd>(data: &Vec<T>, pivot: T) -> usize {
    let mut counter: usize = 0;

    for d in data.iter() {
        if *d > pivot {
            counter += 1
        }
    }

    counter
}