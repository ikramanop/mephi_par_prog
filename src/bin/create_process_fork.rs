use std::env;
use anyhow::anyhow;
use ipc_channel::ipc;
use ipc_channel::ipc::IpcReceiverSet;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::fork;
use wav::BitDepth;
use mephi_par_prog::process_wav_file;

const PIVOT: usize = 1600;
const THREADS: usize = 6;

fn main() -> anyhow::Result<()> {
    let mut rx_set = IpcReceiverSet::new().unwrap();

    for i in 0..THREADS {
        let pid = unsafe { fork() };

        match pid.expect("Fork Failed: Unable to create child process!") {
            Child => {
                let wav_data = process_wav_file(env::var("WAV_FILE_PATH").unwrap())?;

                let mut count = 0;

                match wav_data {
                    BitDepth::Eight(data) => {
                        count += count_diff(&data, PIVOT as u8, i as i32) as i32;
                    }
                    BitDepth::Sixteen(data) => {
                        count += count_diff(&data, PIVOT as i16, i as i32) as i32;
                    }
                    BitDepth::TwentyFour(data) => {
                        count += count_diff(&data, PIVOT as i32, i as i32) as i32;
                    }
                    BitDepth::ThirtyTwoFloat(data) => {
                        count += count_diff(&data, PIVOT as f32, i as i32) as i32;
                    }
                    BitDepth::Empty => {
                        return Err(anyhow!("Empty wav file"));
                    }
                }

                let (tx, rx) = ipc::channel().unwrap();
                rx_set.add(rx).unwrap();

                tx.send(count).unwrap();

                break;
            }
            Parent { child } => {
                let mut count = 0;

                for a in rx_set.select().unwrap().into_iter() {
                    let (_, raw_data) = a.unwrap();
                    let c: i32 = raw_data.to().unwrap();

                    count += c;
                }

                println!("Numbers bigger than {}: {},", PIVOT, count);
            }
        }
    }

    Ok(())
}

fn count_diff<T>(data: &[T], pivot: T, i: i32) -> usize
    where
        T: PartialOrd + Send + Sync + Copy + Clone + 'static,
{
    let mut count = 0;

    let mut j = data.len() as i32 - 1 - i;
    while j >= 0 {
        if data[j as usize] > pivot {
            count += 1
        }
        j -= THREADS as i32;
    }

    count
}
