use anyhow::anyhow;
use std::process::{Child, Command, Stdio};
use mephi_par_prog::process_wav_file;
use std::env;
use std::io::Read;
use wav::BitDepth;

const PIVOT: usize = 1600;
const THREADS: usize = 6;

fn main() -> anyhow::Result<()> {
    let args = get_args();

    if args.len() == 1 {
        // main process
        let mut child_processes = Vec::<Child>::new();

        for i in 0..THREADS {
            let cmd = format!("{:?} {}", env::current_exe().unwrap(), i);

            let child = Command::new(&cmd).stdout(Stdio::piped()).spawn()?;
            child_processes.push(child);
        }

        let mut counter = 0i32;

        let mut count_finished = vec![0; THREADS];
        let mut count_error = vec![0; THREADS];
        loop {
            for (i, child) in child_processes.iter_mut().enumerate() {
                match child.try_wait() {
                    Err(err) => {
                        if count_finished[i] == 1 || count_error[i] == 1 {
                            continue;
                        }
                        println!("Error with process {:?}", err);
                        count_error[i] = 1;
                    }
                    Ok(Some(_status)) => {
                        let mut result = Vec::<u8>::new();
                        child.stdout.as_mut().unwrap().read(&mut result)?;

                        counter += std::str::from_utf8(&result)?.parse::<i32>()?;

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

        let mut count = 0i32;

        match wav_data {
            BitDepth::Eight(data) => {
                count += count_diff(&data, PIVOT as u8, *i) as i32;
            }
            BitDepth::Sixteen(data) => {
                count += count_diff(&data, PIVOT as i16, *i) as i32;
            }
            BitDepth::TwentyFour(data) => {
                count += count_diff(&data, PIVOT as i32, *i) as i32;
            }
            BitDepth::ThirtyTwoFloat(data) => {
                count += count_diff(&data, PIVOT as f32, *i) as i32;
            }
            BitDepth::Empty => {
                return Err(anyhow!("Empty wav file"));
            }
        }

        println!("{}", count)
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
