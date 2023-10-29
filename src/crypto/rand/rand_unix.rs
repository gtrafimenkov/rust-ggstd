// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2010 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Unix cryptographically secure pseudorandom number
//! generator.

use std::io::Read;
use std::sync::atomic;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Mutex;
use std::sync::OnceLock;

const URANDOM_DEVICE: &str = "/dev/urandom";

const DEV_STATE_NOT_INITIALIZED: u32 = 0; // random gen device is not initialized
const DEV_STATE_USING_GET_RANDOM: u32 = 10; // initialized, using get_random
const DEV_STATE_USING_DEV_URANDOM: u32 = 20; // initialized, using /dev/urandom
const DEV_STATE_NO_DEVICE_AVAILABLE: u32 = 30; // no device available
static DEVICE_STATE: atomic::AtomicU32 = atomic::AtomicU32::new(DEV_STATE_NOT_INITIALIZED);

/// DEVICE_INIT mutex protects random device initialization routine.
static DEVICE_INIT: Mutex<()> = Mutex::new(());

/// DEV_URANDOM is an opened /dev/urandom file protected with a mutex.
/// It is initialized only when DEVICE_STATE == DEV_STATE_USING_DEV_URANDOM.
static DEV_URANDOM: OnceLock<Mutex<std::fs::File>> = OnceLock::new();

fn init_random_gen_device() {
    // trying to use get_random
    let mut buf = [0; 1];
    if super::get_random(&mut buf).is_ok() {
        // get_random works
        DEVICE_STATE.store(DEV_STATE_USING_GET_RANDOM, SeqCst);
    } else {
        // trying to open /dev/urandom
        match std::fs::File::open(URANDOM_DEVICE) {
            Ok(f) => {
                assert!(DEV_URANDOM.set(Mutex::new(f)).is_ok());
                DEVICE_STATE.store(DEV_STATE_USING_DEV_URANDOM, SeqCst);
            }
            Err(_) => DEVICE_STATE.store(DEV_STATE_NO_DEVICE_AVAILABLE, SeqCst),
        }
    }
}

/// read_random reads the whole buffer of random data.
pub fn read_random(b: &mut [u8]) -> std::io::Result<()> {
    // Init random generation device on first use.
    // First do atomic check to see if initialization is needed.  If needed, use locking
    // to avoid parallel execution and do another atomic check to be sure that the initialization
    // is still needed.
    if DEVICE_STATE.load(SeqCst) == DEV_STATE_NOT_INITIALIZED {
        let _mutext = DEVICE_INIT.lock().unwrap();
        if DEVICE_STATE.load(SeqCst) == DEV_STATE_NOT_INITIALIZED {
            init_random_gen_device();
        }
    }

    if DEVICE_STATE.load(SeqCst) == DEV_STATE_NO_DEVICE_AVAILABLE {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }

    let read_max = super::get_random_max_read();
    let mut out = b;
    while !out.is_empty() {
        let read = out.len().min(read_max);
        match DEVICE_STATE.load(SeqCst) {
            DEV_STATE_USING_GET_RANDOM => super::get_random(&mut out[..read])?,
            DEV_STATE_USING_DEV_URANDOM => {
                let f = DEV_URANDOM.get().unwrap().lock();
                let mut f = f.unwrap();
                f.read_exact(&mut out[..read])?
            }
            _ => panic!("unreacheable"),
        }
        out = &mut out[read..];
    }
    Ok(())
}
