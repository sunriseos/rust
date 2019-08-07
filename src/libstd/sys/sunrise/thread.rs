use crate::ffi::CStr;
use crate::io;
use crate::sys::{unsupported, Void};
use crate::time::Duration;

pub struct Thread(Void);

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, _p: Box<dyn FnOnce()>)
        -> io::Result<Thread>
    {
        panic!("not supported on sunrise yet")
    }

    pub fn yield_now() {
        panic!("not supported on sunrise yet")
    }

    pub fn set_name(_name: &CStr) {
        panic!("not supported on sunrise yet")
    }

    pub fn sleep(_dur: Duration) {
        panic!("not supported on sunrise yet")
    }

    pub fn join(self) {
        match self.0 {}
    }
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> { None }
    pub unsafe fn init() -> Option<Guard> { None }
}

cfg_if::cfg_if! {
    if #[cfg(target_feature = "atomics")] {
        pub fn my_id() -> u32 {
            panic!("thread ids not implemented on surnise with atomics yet")
        }

        pub fn tcb_get() -> *mut u8 {
            panic!("thread local data not implemented on surnise with atomics yet")
        }

        pub fn tcb_set(_ptr: *mut u8) {
            panic!("thread local data not implemented on surnise with atomics yet")
        }
    } else {
        // stubbed out because no functions actually access these intrinsics
        // unless atomics are enabled
    }
}
