use cust::prelude::*;
use nanorand::{Rng, WyRand};
use std::error::Error;
use anyhow::anyhow;

static PTX: &str = include_str!("../../../resources/calc.ptx");

const THRESHOLD: f32 = 1600f32;

fn main() -> anyhow::Result<()> {
    let _ctx = cust::quick_init()?;

    let module = Module::from_ptx(PTX, &[])?;

    let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

    let func = module.get_function("calc")?;

    let (_, block_size) = func.suggested_launch_configuration(0, 0.into())?;

    let grid_size = (NUMBERS_LEN as u32 + block_size - 1) / block_size;

    let mut out = vec![0.0f32; block_size as usize];
    let out_buf = out.as_slice().as_dbuf()?;

    let data = match wav_data {
        BitDepth::Eight(data) => {
            convert(&data)
        }
        BitDepth::Sixteen(data) => {
            convert(&data)
        }
        BitDepth::TwentyFour(data) => {
            convert(data)
        }
        BitDepth::ThirtyTwoFloat(data) => {
            data
        }
        BitDepth::Empty => {
            return Err(anyhow!("Empty wav file"));
        }
    };

    let lhs_gpu = data.as_slice().as_dbuf()?;

    println!(
        "using {} blocks and {} threads per block",
        grid_size, block_size
    );

    unsafe {
        launch!(
            func<<<grid_size, block_size, 0, stream>>>(
                lhs_gpu.as_device_ptr(),
                lhs_gpu.len(),
                THRESHOLD,
                out_buf.as_device_ptr(),
            )
        )?;
    }

    stream.synchronize()?;

    out_buf.copy_to(&mut out)?;

    let sum = out.iter().sum();

    println!("Numbers bigger than {}: {}", THRESHOLD, sum);

    Ok(())
}

fn convert<T>(a: &[T]) -> Vec<f32> {
    let mut v = vec![];

    for i in a.iter() {
        v.push(i as f32)
    }

    v
}