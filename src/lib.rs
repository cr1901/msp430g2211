#![feature(abi_msp430_interrupt)]
#![doc = "Peripheral access API for MSP430G2211 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate msp430;
#[cfg(feature = "rt")]
extern crate msp430_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT1();
    fn PORT2();
    fn TIMERA1();
    fn TIMERA0();
    fn WDT();
    fn COMPARATORA();
    fn NMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 15] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORT1 },
    Vector { _handler: PORT2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMERA1 },
    Vector { _handler: TIMERA0 },
    Vector { _handler: WDT },
    Vector {
        _handler: COMPARATORA,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: NMI },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "2 - 0xFFE4 Port 1"]
    PORT1 = 2,
    #[doc = "3 - 0xFFE6 Port 2"]
    PORT2 = 3,
    #[doc = "8 - 0xFFF0 Timer A CC1-2, TA"]
    TIMERA1 = 8,
    #[doc = "9 - 0xFFF2 Timer A CC0"]
    TIMERA0 = 9,
    #[doc = "10 - 0xFFF4 Watchdog Timer"]
    WDT = 10,
    #[doc = "11 - 0xFFF6 Comparator A"]
    COMPARATORA = 11,
    #[doc = "14 - 0xFFFC Non-maskable"]
    NMI = 14,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
#[allow(unused_imports)]
use generic::*;
#[cfg(feature = "rt")]
pub use msp430_rt::interrupt;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Special Function"]
pub struct SPECIAL_FUNCTION {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPECIAL_FUNCTION {}
impl SPECIAL_FUNCTION {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const special_function::RegisterBlock {
        0 as *const _
    }
}
impl Deref for SPECIAL_FUNCTION {
    type Target = special_function::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPECIAL_FUNCTION::ptr() }
    }
}
#[doc = "Special Function"]
pub mod special_function;
#[doc = "Port 1/2"]
pub struct PORT_1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_1_2 {}
impl PORT_1_2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_1_2::RegisterBlock {
        0x20 as *const _
    }
}
impl Deref for PORT_1_2 {
    type Target = port_1_2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_1_2::ptr() }
    }
}
#[doc = "Port 1/2"]
pub mod port_1_2;
#[doc = "System Clock"]
pub struct SYSTEM_CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEM_CLOCK {}
impl SYSTEM_CLOCK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_clock::RegisterBlock {
        0x52 as *const _
    }
}
impl Deref for SYSTEM_CLOCK {
    type Target = system_clock::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEM_CLOCK::ptr() }
    }
}
#[doc = "System Clock"]
pub mod system_clock;
#[doc = "Comparator A"]
pub struct COMPARATOR_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMPARATOR_A {}
impl COMPARATOR_A {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comparator_a::RegisterBlock {
        0x58 as *const _
    }
}
impl Deref for COMPARATOR_A {
    type Target = comparator_a::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMPARATOR_A::ptr() }
    }
}
#[doc = "Comparator A"]
pub mod comparator_a;
#[doc = "Calibration Data"]
pub struct CALIBRATION_DATA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CALIBRATION_DATA {}
impl CALIBRATION_DATA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const calibration_data::RegisterBlock {
        0x10fe as *const _
    }
}
impl Deref for CALIBRATION_DATA {
    type Target = calibration_data::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CALIBRATION_DATA::ptr() }
    }
}
#[doc = "Calibration Data"]
pub mod calibration_data;
#[doc = "Watchdog Timer"]
pub struct WATCHDOG_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG_TIMER {}
impl WATCHDOG_TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog_timer::RegisterBlock {
        0x0120 as *const _
    }
}
impl Deref for WATCHDOG_TIMER {
    type Target = watchdog_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG_TIMER::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog_timer;
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x0128 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "Timer A2"]
pub struct TIMER_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_A2 {}
impl TIMER_A2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_a2::RegisterBlock {
        0x012e as *const _
    }
}
impl Deref for TIMER_A2 {
    type Target = timer_a2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_A2::ptr() }
    }
}
#[doc = "Timer A2"]
pub mod timer_a2;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SPECIAL_FUNCTION"]
    pub SPECIAL_FUNCTION: SPECIAL_FUNCTION,
    #[doc = "PORT_1_2"]
    pub PORT_1_2: PORT_1_2,
    #[doc = "SYSTEM_CLOCK"]
    pub SYSTEM_CLOCK: SYSTEM_CLOCK,
    #[doc = "COMPARATOR_A"]
    pub COMPARATOR_A: COMPARATOR_A,
    #[doc = "CALIBRATION_DATA"]
    pub CALIBRATION_DATA: CALIBRATION_DATA,
    #[doc = "WATCHDOG_TIMER"]
    pub WATCHDOG_TIMER: WATCHDOG_TIMER,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "TIMER_A2"]
    pub TIMER_A2: TIMER_A2,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        msp430::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            SPECIAL_FUNCTION: SPECIAL_FUNCTION {
                _marker: PhantomData,
            },
            PORT_1_2: PORT_1_2 {
                _marker: PhantomData,
            },
            SYSTEM_CLOCK: SYSTEM_CLOCK {
                _marker: PhantomData,
            },
            COMPARATOR_A: COMPARATOR_A {
                _marker: PhantomData,
            },
            CALIBRATION_DATA: CALIBRATION_DATA {
                _marker: PhantomData,
            },
            WATCHDOG_TIMER: WATCHDOG_TIMER {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            TIMER_A2: TIMER_A2 {
                _marker: PhantomData,
            },
        }
    }
}
