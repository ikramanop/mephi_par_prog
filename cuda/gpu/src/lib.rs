#![cfg_attr(
target_os = "cuda",
no_std,
feature(register_attr),
register_attr(nvvm_internal)
)]

use cuda_std::prelude::*;

#[kernel]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe fn calc(a: &[f32], threshold: f32, c: *mut f32) {
    let idx1 = thread::index_1d() as usize;
    let idx2 = thread::index() as usize;

    if idx1 < a.len() {
        if a[idx1] > threshold {
            let elem = &mut *c.add(idx2);
            *elem += 1f32;
        }
    }
}
