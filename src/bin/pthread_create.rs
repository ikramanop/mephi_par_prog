use anyhow::anyhow;
use mephi_par_prog::{convert_i16, convert_i32, convert_u8, process_wav_file, ThreadParamsF32};
use std::ffi::c_void;
use std::ptr::{null, null_mut};
use std::sync::mpsc::channel;
use wav::BitDepth;
use libc::pthread_create;

const PIVOT: usize = 1600;
const THREADS: usize = 1;

fn main() -> anyhow::Result<()> {
    let wav_data = process_wav_file(std::env::var("WAV_FILE_PATH").unwrap())?;

    match wav_data {
        BitDepth::Eight(data) => unsafe {
            println!(
                "Numbers bigger than {}: {}, overall {}",
                PIVOT,
                count_diff_f32(&convert_u8(&data)),
                data.len()
            )
        },
        BitDepth::Sixteen(data) => unsafe {
            println!(
                "Numbers bigger than {}: {}, overall {}",
                PIVOT,
                count_diff_f32(&convert_i16(&data)),
                data.len()
            )
        },
        BitDepth::TwentyFour(data) => unsafe {
            println!(
                "Numbers bigger than {}: {}, overall {}",
                PIVOT,
                count_diff_f32(&convert_i32(&data)),
                data.len()
            )
        },
        BitDepth::ThirtyTwoFloat(data) => unsafe {
            println!(
                "Numbers bigger than {}: {}, overall {}",
                PIVOT,
                count_diff_f32(&data),
                data.len()
            )
        },
        BitDepth::Empty => {
            return Err(anyhow!("Empty wav file"));
        }
    }

    Ok(())
}

unsafe fn count_diff_f32(data: &[f32]) -> f32 {
    let mut counter: f32 = 0f32;

    let (tx, rx) = channel();

    for i in 0..THREADS {
        let params = ThreadParamsF32 {
            data: Vec::from(data),
            tx: tx.clone(),
            i,
        };

        let mut id = i.clone() as u64;

        pthread_create(
            &mut id,
            null(),
            thread_func_f32,
            &params as *const _ as *mut c_void,
        );
    }
    drop(tx);

    while let Ok(count) = rx.recv() {
        counter += count;
    }

    counter
}

extern "C" fn thread_func_f32(lpthreadparameter: *mut c_void) -> *mut c_void {
    let params: &mut ThreadParamsF32 = unsafe { &mut *(lpthreadparameter as *mut ThreadParamsF32) };

    let mut count = 0f32;

    let mut j = (params.data.len() - 1 - params.i) as i32;
    while j >= 0 {
        if params.data[j as usize] > PIVOT as f32 {
            count += 1f32
        }
        j -= THREADS as i32;
    }

    params.tx.send(count).unwrap();

    null_mut()
}
