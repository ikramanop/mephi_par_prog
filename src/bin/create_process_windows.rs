use anyhow::anyhow;
use create_process_w::{Child, Command};
use mephi_par_prog::process_wav_file;
use std::env;
use std::process::exit;
use wav::BitDepth;

const PIVOT: usize = 1600;
const THREADS: usize = 6;

fn main() -> anyhow::Result<()> {
    let args = get_args();

    if args.len() == 1 {
        // main process
        let mut child_processes = Vec::<Child>::new();

        for i in 0..THREADS {
            let cmd = format!("{} {}", args[0], i);

            let child = Command::new(&cmd).spawn()?;
            child_processes.push(child);
        }

        let mut counter = 0u32;

        let mut count_finished = vec![0; THREADS];
        let mut count_error = vec![0; THREADS];
        loop {
            for (i, child) in child_processes.iter().enumerate() {
                match child.try_wait() {
                    Err(err) => {
                        if count_finished[i] == 1 || count_error[i] == 1 {
                            continue;
                        }
                        println!("Error with process {:?}", err);
                        count_error[i] = 1;
                    }
                    Ok(Some(status)) => {
                        counter += status.code();

                        count_finished[i] = 1;
                    }
                    Ok(None) => {}
                }
            }

            if count_error.iter().sum::<i32>() == THREADS as i32 {
                return Err(anyhow!("problems with processes %("));
            }

            if count_finished.iter().sum::<i32>() == THREADS as i32 {
                break;
            }
        }

        println!("Numbers bigger than {}: {},", PIVOT, counter);
    } else {
        // child process

        let i = &get_args()[1].parse::<i32>()?;

        let wav_data = process_wav_file(env::var("WAV_FILE_PATH").unwrap())?;

        let mut count = 0f32;

        match wav_data {
            BitDepth::Eight(data) => {
                count += count_diff(&data, PIVOT as u8, *i) as f32;
            }
            BitDepth::Sixteen(data) => {
                count += count_diff(&data, PIVOT as i16, *i) as f32;
            }
            BitDepth::TwentyFour(data) => {
                count += count_diff(&data, PIVOT as i32, *i) as f32;
            }
            BitDepth::ThirtyTwoFloat(data) => {
                count += count_diff(&data, PIVOT as f32, *i) as f32;
            }
            BitDepth::Empty => {
                return Err(anyhow!("Empty wav file"));
            }
        }

        exit(count as i32)
    }

    Ok(())
}

fn get_args() -> Vec<String> {
    let mut args = Vec::new();

    for a in env::args() {
        args.push(a)
    }

    args
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
