//! Implementation for Twizzler

use core::{mem::MaybeUninit, num::NonZeroU32};

use crate::Error;
pub fn getrandom_inner(mut dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    let res = twizzler_rt_abi::random::twz_rt_get_random(
        dest,
        twizzler_rt_abi::random::GetRandomFlags::empty(),
    );
    if res == 0 {
        panic!("failed to fill entropy bytes");
    }
    Ok(())
}

