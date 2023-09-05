/*
 * Copyright 2021-2023 Golden_Water
 * SPDX-License-Identifier: AGPL-3.0-only
 */

use std::sync::Mutex;
use std::{
    ffi::{c_void, CString},
    io,
    os::raw::c_int,
    ptr,
};

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    pub(crate) fn TransformProcessType(
        psn: ProcessSerialNumberPtr,
        transform_state: u32,
    ) -> OSStatus;
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ProcessSerialNumber {
    pub highLongOfPSN: u32,
    pub lowLongOfPSN: u32,
}

pub type ProcessSerialNumberPtr = *mut ProcessSerialNumber;

pub type OSStatus = i32;

pub fn get_psn() -> ProcessSerialNumber {
    ProcessSerialNumber {
        highLongOfPSN: 0,
        lowLongOfPSN: 2,
    }
}
