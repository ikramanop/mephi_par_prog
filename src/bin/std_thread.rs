use anyhow::anyhow;
use mephi_par_prog::process_wav_file;
use std::sync::mpsc::channel;
use std::thread;
use wav::BitDepth;

const PIVOT: i32 = 1600;
const THREADS: i32 = 6;

fn main() -> anyhow::Result<()> {
    let wav_data = process_wav_file(std::env::var("WAV_FILE_PATH").unwrap())?;

    match wav_data {
        BitDepth::Eight(data) => {
            println!(
                "Numbers bigger than {}: {}, overall {}",
                PIVOT,
                count_diff(&data, PIVOT as u8),
                data.len()
            )
        }
        BitDepth::Sixteen(data) => {
            println!(
                "Numbers bigger than {}: {}, overall {}",
                PIVOT,
                count_diff(&data, PIVOT as i16),
                data.len()
            )
        }
        BitDepth::TwentyFour(data) => {
            println!(
                "Numbers bigger than {}: {}, overall {}",
                PIVOT,
                count_diff(&data, PIVOT as i32),
                data.len()
            )a
        }
        BitDepth::ThirtyTwoFloat(data) => {
            println!(
                "Numbers bigger than {}: {}, overall {}",
                PIVOT,
                count_diff(&data, PIVOT as f32),
                data.len()
            )
        }
        BitDepth::Empty => {
            return Err(anyhow!("Empty wav file"));
        }
    }

    Ok(())
}

fn count_diff<T>(data: &[T], pivot: T) -> usize
where
    T: PartialOrd + Send + Sync + Copy + Clone + 'static,
{
    let mut counter: usize = 0;

    let (tx, rx) = channel();

    for i in 0..THREADS {
        let tx = tx.clone();
        let local = Vec::from(data);

        thread::spawn(move || {
            let mut count = 0;

            let mut j = local.len() as i32 - 1 - i;
            while j >= 0 {
                if local[j as usize] > pivot {
                    count += 1
                }
                j -= THREADS;
            }

            tx.send(count).unwrap();
        });
    }
    drop(tx);

    while let Ok(count) = rx.recv() {
        counter += count;
    }

    counter
}
