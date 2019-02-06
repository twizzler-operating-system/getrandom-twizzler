// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for OpenBSD / Bitrig
extern crate libc;

use super::Error;
use std::io;

pub fn getrandom(dest: &mut [u8]) -> Result<(), Error> {
    for chunk in dest.chunks_mut(256) {
        let ret = unsafe {
            libc::getentropy(
                chunk.as_mut_ptr() as *mut libc::c_void,
                chunk.len()
            )
        };
        if ret == -1 {
            return Err(io::Error::last_os_error().into());
        }
    }
    Ok(())
}
