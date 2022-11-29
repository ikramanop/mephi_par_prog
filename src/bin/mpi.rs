extern crate mpi;

use std::env;
use anyhow::anyhow;
use mpi::traits::*;
use wav::BitDepth;
use mephi_par_prog::process_wav_file;

// Запускать только для 6 потоков, иначе работать не будет!!

const PIVOT: usize = 1600;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    if rank == 0 {
        world.barrier();

        let mut count = 0;

        for _ in 1..size {
            let (msg, _) = world.any_process().receive::<i32>();

            count += msg;
        }

        println!("Numbers bigger than {}: {},", PIVOT, count);
    } else {
        let count = process_func(rank, size - 1).unwrap();

        world.process_at_rank(0).send(&count);

        world.barrier();
    }
}

fn process_func(rank: i32, size: i32) -> anyhow::Result<i32> {
    let wav_data = process_wav_file(env::var("WAV_FILE_PATH").unwrap()).unwrap();

    let mut count = 0i32;

    match wav_data {
        BitDepth::Eight(data) => {
            count += count_diff(&data, PIVOT as u8, rank, size) as i32;
        }
        BitDepth::Sixteen(data) => {
            count += count_diff(&data, PIVOT as i16, rank, size) as i32;
        }
        BitDepth::TwentyFour(data) => {
            count += count_diff(&data, PIVOT as i32, rank, size) as i32;
        }
        BitDepth::ThirtyTwoFloat(data) => {
            count += count_diff(&data, PIVOT as f32, rank, size) as i32;
        }
        BitDepth::Empty => {
            return Err(anyhow!("Empty wav file"));
        }
    }

    Ok(count)
}

fn count_diff<T>(data: &[T], pivot: T, i: i32, threads: i32) -> usize
    where
        T: PartialOrd + Send + Sync + Copy + Clone + 'static,
{
    let mut count = 0;

    let mut j = data.len() as i32 - 1 - i;
    while j >= 0 {
        if data[j as usize] > pivot {
            count += 1
        }
        j -= threads;
    }

    count
}