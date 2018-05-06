# ! [ feature ( abi_msp430_interrupt ) ]
# ! [ cfg_attr ( feature = "rt" , feature ( asm ) ) ]
# ! [ cfg_attr ( feature = "rt" , feature ( core_intrinsics ) ) ]
# ! [ cfg_attr ( feature = "rt" , feature ( linkage ) ) ]
# ! [ cfg_attr ( feature = "rt" , feature ( naked_functions ) ) ]
# ! [ cfg_attr ( feature = "rt" , feature ( used ) ) ]
# ! [ cfg_attr ( feature = "rt" , feature ( use_extern_macros ) ) ]
# ! [ doc = "Peripheral access API for MSP430G2211 microcontrollers (generated using svd2rust v0.11.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.11.1/svd2rust/#peripheral-api" ]
# ! [ deny ( missing_docs ) ]
# ! [ deny ( warnings ) ]
# ! [ allow ( non_camel_case_types ) ]
# ! [ feature ( const_fn ) ]
# ! [ no_std ]
#[cfg(feature = "rt")]
extern crate msp430_rt ;
extern crate bare_metal ;
extern crate vcell ;
#[cfg(feature = "rt")]
pub use msp430_rt::default_handler;
use core::ops::Deref;
use bare_metal::Peripheral;
pub use interrupt::Interrupt;
#[doc(hidden)]
pub mod interrupt {
    use bare_metal::Nr;
    #[allow(non_snake_case)]
    #[allow(private_no_mangle_fns)]
    #[cfg(feature = "rt")]
    #[linkage = "weak"]
    #[naked]
    #[no_mangle]
    extern "msp430-interrupt" fn PORT1() {
        unsafe {
            asm ! ( "jmp DEFAULT_HANDLER" :: :: "volatile" );
            ::core::intrinsics::unreachable()
        }
    }
    #[allow(non_snake_case)]
    #[allow(private_no_mangle_fns)]
    #[cfg(feature = "rt")]
    #[linkage = "weak"]
    #[naked]
    #[no_mangle]
    extern "msp430-interrupt" fn PORT2() {
        unsafe {
            asm ! ( "jmp DEFAULT_HANDLER" :: :: "volatile" );
            ::core::intrinsics::unreachable()
        }
    }
    #[allow(non_snake_case)]
    #[allow(private_no_mangle_fns)]
    #[cfg(feature = "rt")]
    #[linkage = "weak"]
    #[naked]
    #[no_mangle]
    extern "msp430-interrupt" fn TIMERA1() {
        unsafe {
            asm ! ( "jmp DEFAULT_HANDLER" :: :: "volatile" );
            ::core::intrinsics::unreachable()
        }
    }
    #[allow(non_snake_case)]
    #[allow(private_no_mangle_fns)]
    #[cfg(feature = "rt")]
    #[linkage = "weak"]
    #[naked]
    #[no_mangle]
    extern "msp430-interrupt" fn TIMERA0() {
        unsafe {
            asm ! ( "jmp DEFAULT_HANDLER" :: :: "volatile" );
            ::core::intrinsics::unreachable()
        }
    }
    #[allow(non_snake_case)]
    #[allow(private_no_mangle_fns)]
    #[cfg(feature = "rt")]
    #[linkage = "weak"]
    #[naked]
    #[no_mangle]
    extern "msp430-interrupt" fn WDT() {
        unsafe {
            asm ! ( "jmp DEFAULT_HANDLER" :: :: "volatile" );
            ::core::intrinsics::unreachable()
        }
    }
    #[allow(non_snake_case)]
    #[allow(private_no_mangle_fns)]
    #[cfg(feature = "rt")]
    #[linkage = "weak"]
    #[naked]
    #[no_mangle]
    extern "msp430-interrupt" fn COMPARATORA() {
        unsafe {
            asm ! ( "jmp DEFAULT_HANDLER" :: :: "volatile" );
            ::core::intrinsics::unreachable()
        }
    }
    #[allow(non_snake_case)]
    #[allow(private_no_mangle_fns)]
    #[cfg(feature = "rt")]
    #[linkage = "weak"]
    #[naked]
    #[no_mangle]
    extern "msp430-interrupt" fn NMI() {
        unsafe {
            asm ! ( "jmp DEFAULT_HANDLER" :: :: "volatile" );
            ::core::intrinsics::unreachable()
        }
    }
    #[allow(private_no_mangle_statics)]
    #[cfg(feature = "rt")]
    #[doc(hidden)]
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    #[used]
    pub static INTERRUPTS: [Option<extern "msp430-interrupt" fn()>; 15] = [
        None,
        None,
        Some(PORT1),
        Some(PORT2),
        None,
        None,
        None,
        None,
        Some(TIMERA1),
        Some(TIMERA0),
        Some(WDT),
        Some(COMPARATORA),
        None,
        None,
        Some(NMI),
    ];
    #[doc = r" Enumeration of all the interrupts"]
    pub enum Interrupt {
        #[doc = "2 - 0xFFE4 Port 1"]
        PORT1,
        #[doc = "3 - 0xFFE6 Port 2"]
        PORT2,
        #[doc = "8 - 0xFFF0 Timer A CC1-2, TA"]
        TIMERA1,
        #[doc = "9 - 0xFFF2 Timer A CC0"]
        TIMERA0,
        #[doc = "10 - 0xFFF4 Watchdog Timer"]
        WDT,
        #[doc = "11 - 0xFFF6 Comparator A"]
        COMPARATORA,
        #[doc = "14 - 0xFFFC Non-maskable"]
        NMI,
    }
    unsafe impl Nr for Interrupt {
        #[inline(always)]
        fn nr(&self) -> u8 {
            match *self {
                Interrupt::PORT1 => 2,
                Interrupt::PORT2 => 3,
                Interrupt::TIMERA1 => 8,
                Interrupt::TIMERA0 => 9,
                Interrupt::WDT => 10,
                Interrupt::COMPARATORA => 11,
                Interrupt::NMI => 14,
            }
        }
    }
    #[cfg(feature = "rt")]
    #[macro_export]
    macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ident = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
}
#[doc = "Special Function"]
pub const SPECIAL_FUNCTION: Peripheral<SPECIAL_FUNCTION> = unsafe { Peripheral::new(0) };
#[doc = "Special Function"]
pub mod special_function {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - Interrupt Enable 1"]
        pub ie1: IE1,
        _reserved0: [u8; 1usize],
        #[doc = "0x02 - Interrupt Flag 1"]
        pub ifg1: IFG1,
    }
    #[doc = "Interrupt Enable 1"]
    pub struct IE1 {
        register: VolatileCell<u8>,
    }
    #[doc = "Interrupt Enable 1"]
    pub mod ie1 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::IE1 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `WDTIE`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTIER {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTIER {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTIER::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTIER {
                match value {
                    i => WDTIER::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `OFIE`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum OFIER {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl OFIER {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    OFIER::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> OFIER {
                match value {
                    i => OFIER::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `NMIIE`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum NMIIER {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl NMIIER {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    NMIIER::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> NMIIER {
                match value {
                    i => NMIIER::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `ACCVIE`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum ACCVIER {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl ACCVIER {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    ACCVIER::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> ACCVIER {
                match value {
                    i => ACCVIER::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `WDTIE`"]
        pub enum WDTIEW { }
        impl WDTIEW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTIEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTIEW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTIEW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `OFIE`"]
        pub enum OFIEW { }
        impl OFIEW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _OFIEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _OFIEW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: OFIEW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `NMIIE`"]
        pub enum NMIIEW { }
        impl NMIIEW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _NMIIEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _NMIIEW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: NMIIEW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `ACCVIE`"]
        pub enum ACCVIEW { }
        impl ACCVIEW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _ACCVIEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _ACCVIEW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: ACCVIEW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - Watchdog Interrupt Enable"]
            #[inline(always)]
            pub fn wdtie(&self) -> WDTIER {
                WDTIER::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
            #[inline(always)]
            pub fn ofie(&self) -> OFIER {
                OFIER::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - NMI Interrupt Enable"]
            #[inline(always)]
            pub fn nmiie(&self) -> NMIIER {
                NMIIER::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
            #[inline(always)]
            pub fn accvie(&self) -> ACCVIER {
                ACCVIER::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Watchdog Interrupt Enable"]
            #[inline(always)]
            pub fn wdtie(&mut self) -> _WDTIEW {
                _WDTIEW { w: self }
            }
            #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
            #[inline(always)]
            pub fn ofie(&mut self) -> _OFIEW {
                _OFIEW { w: self }
            }
            #[doc = "Bit 4 - NMI Interrupt Enable"]
            #[inline(always)]
            pub fn nmiie(&mut self) -> _NMIIEW {
                _NMIIEW { w: self }
            }
            #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
            #[inline(always)]
            pub fn accvie(&mut self) -> _ACCVIEW {
                _ACCVIEW { w: self }
            }
        }
    }
    #[doc = "Interrupt Flag 1"]
    pub struct IFG1 {
        register: VolatileCell<u8>,
    }
    #[doc = "Interrupt Flag 1"]
    pub mod ifg1 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::IFG1 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `WDTIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTIFGR {
                match value {
                    i => WDTIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `OFIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum OFIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl OFIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    OFIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> OFIFGR {
                match value {
                    i => OFIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `PORIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum PORIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl PORIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    PORIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> PORIFGR {
                match value {
                    i => PORIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `RSTIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum RSTIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl RSTIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    RSTIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> RSTIFGR {
                match value {
                    i => RSTIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `NMIIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum NMIIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl NMIIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    NMIIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> NMIIFGR {
                match value {
                    i => NMIIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `WDTIFG`"]
        pub enum WDTIFGW { }
        impl WDTIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `OFIFG`"]
        pub enum OFIFGW { }
        impl OFIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _OFIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _OFIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: OFIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `PORIFG`"]
        pub enum PORIFGW { }
        impl PORIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _PORIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _PORIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: PORIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `RSTIFG`"]
        pub enum RSTIFGW { }
        impl RSTIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _RSTIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _RSTIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: RSTIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `NMIIFG`"]
        pub enum NMIIFGW { }
        impl NMIIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _NMIIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _NMIIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: NMIIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - Watchdog Interrupt Flag"]
            #[inline(always)]
            pub fn wdtifg(&self) -> WDTIFGR {
                WDTIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
            #[inline(always)]
            pub fn ofifg(&self) -> OFIFGR {
                OFIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - Power On Interrupt Flag"]
            #[inline(always)]
            pub fn porifg(&self) -> PORIFGR {
                PORIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - Reset Interrupt Flag"]
            #[inline(always)]
            pub fn rstifg(&self) -> RSTIFGR {
                RSTIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - NMI Interrupt Flag"]
            #[inline(always)]
            pub fn nmiifg(&self) -> NMIIFGR {
                NMIIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Watchdog Interrupt Flag"]
            #[inline(always)]
            pub fn wdtifg(&mut self) -> _WDTIFGW {
                _WDTIFGW { w: self }
            }
            #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
            #[inline(always)]
            pub fn ofifg(&mut self) -> _OFIFGW {
                _OFIFGW { w: self }
            }
            #[doc = "Bit 2 - Power On Interrupt Flag"]
            #[inline(always)]
            pub fn porifg(&mut self) -> _PORIFGW {
                _PORIFGW { w: self }
            }
            #[doc = "Bit 3 - Reset Interrupt Flag"]
            #[inline(always)]
            pub fn rstifg(&mut self) -> _RSTIFGW {
                _RSTIFGW { w: self }
            }
            #[doc = "Bit 4 - NMI Interrupt Flag"]
            #[inline(always)]
            pub fn nmiifg(&mut self) -> _NMIIFGW {
                _NMIIFGW { w: self }
            }
        }
    }
}
#[doc = "Special Function"]
pub struct SPECIAL_FUNCTION {
    register_block: special_function::RegisterBlock,
}
impl Deref for SPECIAL_FUNCTION {
    type Target = special_function::RegisterBlock;
    fn deref(&self) -> &special_function::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Comparator A"]
pub const COMPARATOR_A: Peripheral<COMPARATOR_A> = unsafe { Peripheral::new(88) };
#[doc = "Comparator A"]
pub mod comparator_a {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        _reserved0: [u8; 1usize],
        #[doc = "0x01 - Comparator A Control 1"]
        pub cactl1: CACTL1,
        #[doc = "0x02 - Comparator A Control 2"]
        pub cactl2: CACTL2,
        #[doc = "0x03 - Comparator A Port Disable"]
        pub capd: CAPD,
    }
    #[doc = "Comparator A Control 1"]
    pub struct CACTL1 {
        register: VolatileCell<u8>,
    }
    #[doc = "Comparator A Control 1"]
    pub mod cactl1 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::CACTL1 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `CAIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAIFGR {
                match value {
                    i => CAIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAIE`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAIER {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAIER {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAIER::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAIER {
                match value {
                    i => CAIER::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAIES`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAIESR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAIESR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAIESR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAIESR {
                match value {
                    i => CAIESR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAON`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAONR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAONR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAONR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAONR {
                match value {
                    i => CAONR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAREF`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAREFR {
            #[doc = "Comp. A Int. Ref. Select 0 : Off"]
            CAREF_0,
            #[doc = "Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
            CAREF_1,
            #[doc = "Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
            CAREF_2,
            #[doc = "Comp. A Int. Ref. Select 3 : Vt"]
            CAREF_3,
        }
        impl CAREFR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    CAREFR::CAREF_0 => 0,
                    CAREFR::CAREF_1 => 1,
                    CAREFR::CAREF_2 => 2,
                    CAREFR::CAREF_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> CAREFR {
                match value {
                    0 => CAREFR::CAREF_0,
                    1 => CAREFR::CAREF_1,
                    2 => CAREFR::CAREF_2,
                    3 => CAREFR::CAREF_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `CAREF_0`"]
            #[inline(always)]
            pub fn is_caref_0(&self) -> bool {
                *self == CAREFR::CAREF_0
            }
            #[doc = "Checks if the value of the field is `CAREF_1`"]
            #[inline(always)]
            pub fn is_caref_1(&self) -> bool {
                *self == CAREFR::CAREF_1
            }
            #[doc = "Checks if the value of the field is `CAREF_2`"]
            #[inline(always)]
            pub fn is_caref_2(&self) -> bool {
                *self == CAREFR::CAREF_2
            }
            #[doc = "Checks if the value of the field is `CAREF_3`"]
            #[inline(always)]
            pub fn is_caref_3(&self) -> bool {
                *self == CAREFR::CAREF_3
            }
        }
        #[doc = "Possible values of the field `CARSEL`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CARSELR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CARSELR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CARSELR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CARSELR {
                match value {
                    i => CARSELR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAEX`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAEXR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAEXR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAEXR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAEXR {
                match value {
                    i => CAEXR::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `CAIFG`"]
        pub enum CAIFGW { }
        impl CAIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAIE`"]
        pub enum CAIEW { }
        impl CAIEW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAIEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAIEW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAIEW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAIES`"]
        pub enum CAIESW { }
        impl CAIESW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAIESW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAIESW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAIESW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAON`"]
        pub enum CAONW { }
        impl CAONW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAONW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAONW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAONW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAREF`"]
        pub enum CAREFW {
            #[doc = "Comp. A Int. Ref. Select 0 : Off"]
            CAREF_0,
            #[doc = "Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
            CAREF_1,
            #[doc = "Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
            CAREF_2,
            #[doc = "Comp. A Int. Ref. Select 3 : Vt"]
            CAREF_3,
        }
        impl CAREFW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    CAREFW::CAREF_0 => 0,
                    CAREFW::CAREF_1 => 1,
                    CAREFW::CAREF_2 => 2,
                    CAREFW::CAREF_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAREFW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAREFW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAREFW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Comp. A Int. Ref. Select 0 : Off"]
            #[inline(always)]
            pub fn caref_0(self) -> &'a mut W {
                self.variant(CAREFW::CAREF_0)
            }
            #[doc = "Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
            #[inline(always)]
            pub fn caref_1(self) -> &'a mut W {
                self.variant(CAREFW::CAREF_1)
            }
            #[doc = "Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
            #[inline(always)]
            pub fn caref_2(self) -> &'a mut W {
                self.variant(CAREFW::CAREF_2)
            }
            #[doc = "Comp. A Int. Ref. Select 3 : Vt"]
            #[inline(always)]
            pub fn caref_3(self) -> &'a mut W {
                self.variant(CAREFW::CAREF_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CARSEL`"]
        pub enum CARSELW { }
        impl CARSELW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CARSELW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CARSELW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CARSELW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAEX`"]
        pub enum CAEXW { }
        impl CAEXW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAEXW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAEXW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAEXW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - Comp. A Interrupt Flag"]
            #[inline(always)]
            pub fn caifg(&self) -> CAIFGR {
                CAIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - Comp. A Interrupt Enable"]
            #[inline(always)]
            pub fn caie(&self) -> CAIER {
                CAIER::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
            #[inline(always)]
            pub fn caies(&self) -> CAIESR {
                CAIESR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - Comp. A enable"]
            #[inline(always)]
            pub fn caon(&self) -> CAONR {
                CAONR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
            #[inline(always)]
            pub fn caref(&self) -> CAREFR {
                CAREFR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) as u8
                })
            }
            #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
            #[inline(always)]
            pub fn carsel(&self) -> CARSELR {
                CARSELR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - Comp. A Exchange Inputs"]
            #[inline(always)]
            pub fn caex(&self) -> CAEXR {
                CAEXR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Comp. A Interrupt Flag"]
            #[inline(always)]
            pub fn caifg(&mut self) -> _CAIFGW {
                _CAIFGW { w: self }
            }
            #[doc = "Bit 1 - Comp. A Interrupt Enable"]
            #[inline(always)]
            pub fn caie(&mut self) -> _CAIEW {
                _CAIEW { w: self }
            }
            #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
            #[inline(always)]
            pub fn caies(&mut self) -> _CAIESW {
                _CAIESW { w: self }
            }
            #[doc = "Bit 3 - Comp. A enable"]
            #[inline(always)]
            pub fn caon(&mut self) -> _CAONW {
                _CAONW { w: self }
            }
            #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
            #[inline(always)]
            pub fn caref(&mut self) -> _CAREFW {
                _CAREFW { w: self }
            }
            #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
            #[inline(always)]
            pub fn carsel(&mut self) -> _CARSELW {
                _CARSELW { w: self }
            }
            #[doc = "Bit 7 - Comp. A Exchange Inputs"]
            #[inline(always)]
            pub fn caex(&mut self) -> _CAEXW {
                _CAEXW { w: self }
            }
        }
    }
    #[doc = "Comparator A Control 2"]
    pub struct CACTL2 {
        register: VolatileCell<u8>,
    }
    #[doc = "Comparator A Control 2"]
    pub mod cactl2 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::CACTL2 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `CAOUT`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAOUTR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAOUTR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAOUTR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAOUTR {
                match value {
                    i => CAOUTR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAF`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAFR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAFR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAFR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAFR {
                match value {
                    i => CAFR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2CA0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2CA0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2CA0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2CA0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2CA0R {
                match value {
                    i => P2CA0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2CA1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2CA1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2CA1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2CA1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2CA1R {
                match value {
                    i => P2CA1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2CA2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2CA2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2CA2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2CA2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2CA2R {
                match value {
                    i => P2CA2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2CA3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2CA3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2CA3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2CA3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2CA3R {
                match value {
                    i => P2CA3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2CA4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2CA4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2CA4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2CA4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2CA4R {
                match value {
                    i => P2CA4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CASHORT`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CASHORTR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CASHORTR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CASHORTR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CASHORTR {
                match value {
                    i => CASHORTR::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `CAOUT`"]
        pub enum CAOUTW { }
        impl CAOUTW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAOUTW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAOUTW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAOUTW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAF`"]
        pub enum CAFW { }
        impl CAFW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAFW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAFW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAFW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2CA0`"]
        pub enum P2CA0W { }
        impl P2CA0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2CA0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2CA0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2CA0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2CA1`"]
        pub enum P2CA1W { }
        impl P2CA1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2CA1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2CA1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2CA1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2CA2`"]
        pub enum P2CA2W { }
        impl P2CA2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2CA2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2CA2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2CA2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2CA3`"]
        pub enum P2CA3W { }
        impl P2CA3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2CA3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2CA3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2CA3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2CA4`"]
        pub enum P2CA4W { }
        impl P2CA4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2CA4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2CA4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2CA4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CASHORT`"]
        pub enum CASHORTW { }
        impl CASHORTW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CASHORTW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CASHORTW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CASHORTW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - Comp. A Output"]
            #[inline(always)]
            pub fn caout(&self) -> CAOUTR {
                CAOUTR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - Comp. A Enable Output Filter"]
            #[inline(always)]
            pub fn caf(&self) -> CAFR {
                CAFR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - Comp. A +Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca0(&self) -> P2CA0R {
                P2CA0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - Comp. A -Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca1(&self) -> P2CA1R {
                P2CA1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - Comp. A -Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca2(&self) -> P2CA2R {
                P2CA2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - Comp. A -Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca3(&self) -> P2CA3R {
                P2CA3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - Comp. A +Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca4(&self) -> P2CA4R {
                P2CA4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
            #[inline(always)]
            pub fn cashort(&self) -> CASHORTR {
                CASHORTR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Comp. A Output"]
            #[inline(always)]
            pub fn caout(&mut self) -> _CAOUTW {
                _CAOUTW { w: self }
            }
            #[doc = "Bit 1 - Comp. A Enable Output Filter"]
            #[inline(always)]
            pub fn caf(&mut self) -> _CAFW {
                _CAFW { w: self }
            }
            #[doc = "Bit 2 - Comp. A +Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca0(&mut self) -> _P2CA0W {
                _P2CA0W { w: self }
            }
            #[doc = "Bit 3 - Comp. A -Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca1(&mut self) -> _P2CA1W {
                _P2CA1W { w: self }
            }
            #[doc = "Bit 4 - Comp. A -Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca2(&mut self) -> _P2CA2W {
                _P2CA2W { w: self }
            }
            #[doc = "Bit 5 - Comp. A -Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca3(&mut self) -> _P2CA3W {
                _P2CA3W { w: self }
            }
            #[doc = "Bit 6 - Comp. A +Terminal Multiplexer"]
            #[inline(always)]
            pub fn p2ca4(&mut self) -> _P2CA4W {
                _P2CA4W { w: self }
            }
            #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
            #[inline(always)]
            pub fn cashort(&mut self) -> _CASHORTW {
                _CASHORTW { w: self }
            }
        }
    }
    #[doc = "Comparator A Port Disable"]
    pub struct CAPD {
        register: VolatileCell<u8>,
    }
    #[doc = "Comparator A Port Disable"]
    pub mod capd {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::CAPD {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `CAPD0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPD0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPD0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPD0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPD0R {
                match value {
                    i => CAPD0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAPD1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPD1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPD1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPD1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPD1R {
                match value {
                    i => CAPD1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAPD2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPD2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPD2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPD2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPD2R {
                match value {
                    i => CAPD2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAPD3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPD3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPD3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPD3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPD3R {
                match value {
                    i => CAPD3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAPD4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPD4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPD4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPD4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPD4R {
                match value {
                    i => CAPD4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAPD5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPD5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPD5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPD5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPD5R {
                match value {
                    i => CAPD5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAPD6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPD6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPD6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPD6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPD6R {
                match value {
                    i => CAPD6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CAPD7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPD7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPD7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPD7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPD7R {
                match value {
                    i => CAPD7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `CAPD0`"]
        pub enum CAPD0W { }
        impl CAPD0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPD0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPD0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPD0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAPD1`"]
        pub enum CAPD1W { }
        impl CAPD1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPD1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPD1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPD1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAPD2`"]
        pub enum CAPD2W { }
        impl CAPD2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPD2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPD2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPD2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAPD3`"]
        pub enum CAPD3W { }
        impl CAPD3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPD3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPD3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPD3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAPD4`"]
        pub enum CAPD4W { }
        impl CAPD4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPD4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPD4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPD4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAPD5`"]
        pub enum CAPD5W { }
        impl CAPD5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPD5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPD5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPD5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAPD6`"]
        pub enum CAPD6W { }
        impl CAPD6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPD6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPD6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPD6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAPD7`"]
        pub enum CAPD7W { }
        impl CAPD7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPD7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPD7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPD7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
            #[inline(always)]
            pub fn capd0(&self) -> CAPD0R {
                CAPD0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
            #[inline(always)]
            pub fn capd1(&self) -> CAPD1R {
                CAPD1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
            #[inline(always)]
            pub fn capd2(&self) -> CAPD2R {
                CAPD2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
            #[inline(always)]
            pub fn capd3(&self) -> CAPD3R {
                CAPD3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
            #[inline(always)]
            pub fn capd4(&self) -> CAPD4R {
                CAPD4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
            #[inline(always)]
            pub fn capd5(&self) -> CAPD5R {
                CAPD5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
            #[inline(always)]
            pub fn capd6(&self) -> CAPD6R {
                CAPD6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
            #[inline(always)]
            pub fn capd7(&self) -> CAPD7R {
                CAPD7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
            #[inline(always)]
            pub fn capd0(&mut self) -> _CAPD0W {
                _CAPD0W { w: self }
            }
            #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
            #[inline(always)]
            pub fn capd1(&mut self) -> _CAPD1W {
                _CAPD1W { w: self }
            }
            #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
            #[inline(always)]
            pub fn capd2(&mut self) -> _CAPD2W {
                _CAPD2W { w: self }
            }
            #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
            #[inline(always)]
            pub fn capd3(&mut self) -> _CAPD3W {
                _CAPD3W { w: self }
            }
            #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
            #[inline(always)]
            pub fn capd4(&mut self) -> _CAPD4W {
                _CAPD4W { w: self }
            }
            #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
            #[inline(always)]
            pub fn capd5(&mut self) -> _CAPD5W {
                _CAPD5W { w: self }
            }
            #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
            #[inline(always)]
            pub fn capd6(&mut self) -> _CAPD6W {
                _CAPD6W { w: self }
            }
            #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
            #[inline(always)]
            pub fn capd7(&mut self) -> _CAPD7W {
                _CAPD7W { w: self }
            }
        }
    }
}
#[doc = "Comparator A"]
pub struct COMPARATOR_A {
    register_block: comparator_a::RegisterBlock,
}
impl Deref for COMPARATOR_A {
    type Target = comparator_a::RegisterBlock;
    fn deref(&self) -> &comparator_a::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 1/2"]
pub const PORT_1_2: Peripheral<PORT_1_2> = unsafe { Peripheral::new(32) };
#[doc = "Port 1/2"]
pub mod port_1_2 {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - Port 1 Input"]
        pub p1in: P1IN,
        #[doc = "0x01 - Port 1 Output"]
        pub p1out: P1OUT,
        #[doc = "0x02 - Port 1 Direction"]
        pub p1dir: P1DIR,
        #[doc = "0x03 - Port 1 Interrupt Flag"]
        pub p1ifg: P1IFG,
        #[doc = "0x04 - Port 1 Interrupt Edge Select"]
        pub p1ies: P1IES,
        #[doc = "0x05 - Port 1 Interrupt Enable"]
        pub p1ie: P1IE,
        #[doc = "0x06 - Port 1 Selection"]
        pub p1sel: P1SEL,
        #[doc = "0x07 - Port 1 Resistor Enable"]
        pub p1ren: P1REN,
        #[doc = "0x08 - Port 2 Input"]
        pub p2in: P2IN,
        #[doc = "0x09 - Port 2 Output"]
        pub p2out: P2OUT,
        #[doc = "0x0a - Port 2 Direction"]
        pub p2dir: P2DIR,
        #[doc = "0x0b - Port 2 Interrupt Flag"]
        pub p2ifg: P2IFG,
        #[doc = "0x0c - Port 2 Interrupt Edge Select"]
        pub p2ies: P2IES,
        #[doc = "0x0d - Port 2 Interrupt Enable"]
        pub p2ie: P2IE,
        #[doc = "0x0e - Port 2 Selection"]
        pub p2sel: P2SEL,
        #[doc = "0x0f - Port 2 Resistor Enable"]
        pub p2ren: P2REN,
    }
    #[doc = "Port 1 Input"]
    pub struct P1IN {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 1 Input"]
    pub mod p1in {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P1IN {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 1 Output"]
    pub struct P1OUT {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 1 Output"]
    pub mod p1out {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P1OUT {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 1 Direction"]
    pub struct P1DIR {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 1 Direction"]
    pub mod p1dir {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P1DIR {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 1 Interrupt Flag"]
    pub struct P1IFG {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 1 Interrupt Flag"]
    pub mod p1ifg {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P1IFG {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 1 Interrupt Edge Select"]
    pub struct P1IES {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 1 Interrupt Edge Select"]
    pub mod p1ies {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P1IES {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 1 Interrupt Enable"]
    pub struct P1IE {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 1 Interrupt Enable"]
    pub mod p1ie {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P1IE {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 1 Selection"]
    pub struct P1SEL {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 1 Selection"]
    pub mod p1sel {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P1SEL {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 1 Resistor Enable"]
    pub struct P1REN {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 1 Resistor Enable"]
    pub mod p1ren {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P1REN {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 2 Input"]
    pub struct P2IN {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 2 Input"]
    pub mod p2in {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P2IN {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 2 Output"]
    pub struct P2OUT {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 2 Output"]
    pub mod p2out {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P2OUT {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 2 Direction"]
    pub struct P2DIR {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 2 Direction"]
    pub mod p2dir {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P2DIR {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 2 Interrupt Flag"]
    pub struct P2IFG {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 2 Interrupt Flag"]
    pub mod p2ifg {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P2IFG {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 2 Interrupt Edge Select"]
    pub struct P2IES {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 2 Interrupt Edge Select"]
    pub mod p2ies {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P2IES {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 2 Interrupt Enable"]
    pub struct P2IE {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 2 Interrupt Enable"]
    pub mod p2ie {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P2IE {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 2 Selection"]
    pub struct P2SEL {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 2 Selection"]
    pub mod p2sel {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P2SEL {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
    #[doc = "Port 2 Resistor Enable"]
    pub struct P2REN {
        register: VolatileCell<u8>,
    }
    #[doc = "Port 2 Resistor Enable"]
    pub mod p2ren {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::P2REN {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `P0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P0R {
                match value {
                    i => P0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P1R {
                match value {
                    i => P1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P2R {
                match value {
                    i => P2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P3R {
                match value {
                    i => P3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P4R {
                match value {
                    i => P4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P5R {
                match value {
                    i => P5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P6`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P6R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P6R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P6R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P6R {
                match value {
                    i => P6R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `P7`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum P7R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl P7R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    P7R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> P7R {
                match value {
                    i => P7R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `P0`"]
        pub enum P0W { }
        impl P0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P1`"]
        pub enum P1W { }
        impl P1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P2`"]
        pub enum P2W { }
        impl P2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P3`"]
        pub enum P3W { }
        impl P3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P4`"]
        pub enum P4W { }
        impl P4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P5`"]
        pub enum P5W { }
        impl P5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P6`"]
        pub enum P6W { }
        impl P6W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P6W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P6W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P6W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `P7`"]
        pub enum P7W { }
        impl P7W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _P7W<'a> {
            w: &'a mut W,
        }
        impl<'a> _P7W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: P7W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&self) -> P0R {
                P0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&self) -> P1R {
                P1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&self) -> P2R {
                P2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&self) -> P3R {
                P3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&self) -> P4R {
                P4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&self) -> P5R {
                P5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&self) -> P6R {
                P6R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&self) -> P7R {
                P7R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - P0"]
            #[inline(always)]
            pub fn p0(&mut self) -> _P0W {
                _P0W { w: self }
            }
            #[doc = "Bit 1 - P1"]
            #[inline(always)]
            pub fn p1(&mut self) -> _P1W {
                _P1W { w: self }
            }
            #[doc = "Bit 2 - P2"]
            #[inline(always)]
            pub fn p2(&mut self) -> _P2W {
                _P2W { w: self }
            }
            #[doc = "Bit 3 - P3"]
            #[inline(always)]
            pub fn p3(&mut self) -> _P3W {
                _P3W { w: self }
            }
            #[doc = "Bit 4 - P4"]
            #[inline(always)]
            pub fn p4(&mut self) -> _P4W {
                _P4W { w: self }
            }
            #[doc = "Bit 5 - P5"]
            #[inline(always)]
            pub fn p5(&mut self) -> _P5W {
                _P5W { w: self }
            }
            #[doc = "Bit 6 - P6"]
            #[inline(always)]
            pub fn p6(&mut self) -> _P6W {
                _P6W { w: self }
            }
            #[doc = "Bit 7 - P7"]
            #[inline(always)]
            pub fn p7(&mut self) -> _P7W {
                _P7W { w: self }
            }
        }
    }
}
#[doc = "Port 1/2"]
pub struct PORT_1_2 {
    register_block: port_1_2::RegisterBlock,
}
impl Deref for PORT_1_2 {
    type Target = port_1_2::RegisterBlock;
    fn deref(&self) -> &port_1_2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flash"]
pub const FLASH: Peripheral<FLASH> = unsafe { Peripheral::new(296) };
#[doc = "Flash"]
pub mod flash {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - FLASH Control 1"]
        pub fctl1: FCTL1,
        #[doc = "0x02 - FLASH Control 2"]
        pub fctl2: FCTL2,
        #[doc = "0x04 - FLASH Control 3"]
        pub fctl3: FCTL3,
    }
    #[doc = "FLASH Control 1"]
    pub struct FCTL1 {
        register: VolatileCell<u16>,
    }
    #[doc = "FLASH Control 1"]
    pub mod fctl1 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::FCTL1 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `ERASE`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum ERASER {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl ERASER {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    ERASER::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> ERASER {
                match value {
                    i => ERASER::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `MERAS`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum MERASR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl MERASR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    MERASR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> MERASR {
                match value {
                    i => MERASR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `WRT`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WRTR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WRTR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WRTR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WRTR {
                match value {
                    i => WRTR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `BLKWRT`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum BLKWRTR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl BLKWRTR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    BLKWRTR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> BLKWRTR {
                match value {
                    i => BLKWRTR::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `ERASE`"]
        pub enum ERASEW { }
        impl ERASEW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _ERASEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _ERASEW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: ERASEW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `MERAS`"]
        pub enum MERASW { }
        impl MERASW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _MERASW<'a> {
            w: &'a mut W,
        }
        impl<'a> _MERASW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: MERASW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `WRT`"]
        pub enum WRTW { }
        impl WRTW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WRTW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WRTW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WRTW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `BLKWRT`"]
        pub enum BLKWRTW { }
        impl BLKWRTW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _BLKWRTW<'a> {
            w: &'a mut W,
        }
        impl<'a> _BLKWRTW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: BLKWRTW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
            #[doc = "Bit 1 - Enable bit for Flash segment erase"]
            #[inline(always)]
            pub fn erase(&self) -> ERASER {
                ERASER::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 2 - Enable bit for Flash mass erase"]
            #[inline(always)]
            pub fn meras(&self) -> MERASR {
                MERASR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 6 - Enable bit for Flash write"]
            #[inline(always)]
            pub fn wrt(&self) -> WRTR {
                WRTR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 7 - Enable bit for Flash segment write"]
            #[inline(always)]
            pub fn blkwrt(&self) -> BLKWRTR {
                BLKWRTR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 1 - Enable bit for Flash segment erase"]
            #[inline(always)]
            pub fn erase(&mut self) -> _ERASEW {
                _ERASEW { w: self }
            }
            #[doc = "Bit 2 - Enable bit for Flash mass erase"]
            #[inline(always)]
            pub fn meras(&mut self) -> _MERASW {
                _MERASW { w: self }
            }
            #[doc = "Bit 6 - Enable bit for Flash write"]
            #[inline(always)]
            pub fn wrt(&mut self) -> _WRTW {
                _WRTW { w: self }
            }
            #[doc = "Bit 7 - Enable bit for Flash segment write"]
            #[inline(always)]
            pub fn blkwrt(&mut self) -> _BLKWRTW {
                _BLKWRTW { w: self }
            }
        }
    }
    #[doc = "FLASH Control 2"]
    pub struct FCTL2 {
        register: VolatileCell<u16>,
    }
    #[doc = "FLASH Control 2"]
    pub mod fctl2 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::FCTL2 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `FN0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FN0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl FN0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    FN0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> FN0R {
                match value {
                    i => FN0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `FN1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FN1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl FN1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    FN1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> FN1R {
                match value {
                    i => FN1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `FN2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FN2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl FN2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    FN2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> FN2R {
                match value {
                    i => FN2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `FN3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FN3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl FN3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    FN3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> FN3R {
                match value {
                    i => FN3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `FN4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FN4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl FN4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    FN4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> FN4R {
                match value {
                    i => FN4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `FN5`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FN5R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl FN5R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    FN5R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> FN5R {
                match value {
                    i => FN5R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `FSSEL`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FSSELR {
            #[doc = "Flash clock select: 0 - ACLK"]
            FSSEL_0,
            #[doc = "Flash clock select: 1 - MCLK"]
            FSSEL_1,
            #[doc = "Flash clock select: 2 - SMCLK"]
            FSSEL_2,
            #[doc = "Flash clock select: 3 - SMCLK"]
            FSSEL_3,
        }
        impl FSSELR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    FSSELR::FSSEL_0 => 0,
                    FSSELR::FSSEL_1 => 1,
                    FSSELR::FSSEL_2 => 2,
                    FSSELR::FSSEL_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> FSSELR {
                match value {
                    0 => FSSELR::FSSEL_0,
                    1 => FSSELR::FSSEL_1,
                    2 => FSSELR::FSSEL_2,
                    3 => FSSELR::FSSEL_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `FSSEL_0`"]
            #[inline(always)]
            pub fn is_fssel_0(&self) -> bool {
                *self == FSSELR::FSSEL_0
            }
            #[doc = "Checks if the value of the field is `FSSEL_1`"]
            #[inline(always)]
            pub fn is_fssel_1(&self) -> bool {
                *self == FSSELR::FSSEL_1
            }
            #[doc = "Checks if the value of the field is `FSSEL_2`"]
            #[inline(always)]
            pub fn is_fssel_2(&self) -> bool {
                *self == FSSELR::FSSEL_2
            }
            #[doc = "Checks if the value of the field is `FSSEL_3`"]
            #[inline(always)]
            pub fn is_fssel_3(&self) -> bool {
                *self == FSSELR::FSSEL_3
            }
        }
        #[doc = "Values that can be written to the field `FN0`"]
        pub enum FN0W { }
        impl FN0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _FN0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _FN0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FN0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `FN1`"]
        pub enum FN1W { }
        impl FN1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _FN1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _FN1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FN1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `FN2`"]
        pub enum FN2W { }
        impl FN2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _FN2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _FN2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FN2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `FN3`"]
        pub enum FN3W { }
        impl FN3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _FN3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _FN3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FN3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `FN4`"]
        pub enum FN4W { }
        impl FN4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _FN4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _FN4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FN4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `FN5`"]
        pub enum FN5W { }
        impl FN5W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _FN5W<'a> {
            w: &'a mut W,
        }
        impl<'a> _FN5W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FN5W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `FSSEL`"]
        pub enum FSSELW {
            #[doc = "Flash clock select: 0 - ACLK"]
            FSSEL_0,
            #[doc = "Flash clock select: 1 - MCLK"]
            FSSEL_1,
            #[doc = "Flash clock select: 2 - SMCLK"]
            FSSEL_2,
            #[doc = "Flash clock select: 3 - SMCLK"]
            FSSEL_3,
        }
        impl FSSELW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    FSSELW::FSSEL_0 => 0,
                    FSSELW::FSSEL_1 => 1,
                    FSSELW::FSSEL_2 => 2,
                    FSSELW::FSSEL_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _FSSELW<'a> {
            w: &'a mut W,
        }
        impl<'a> _FSSELW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FSSELW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Flash clock select: 0 - ACLK"]
            #[inline(always)]
            pub fn fssel_0(self) -> &'a mut W {
                self.variant(FSSELW::FSSEL_0)
            }
            #[doc = "Flash clock select: 1 - MCLK"]
            #[inline(always)]
            pub fn fssel_1(self) -> &'a mut W {
                self.variant(FSSELW::FSSEL_1)
            }
            #[doc = "Flash clock select: 2 - SMCLK"]
            #[inline(always)]
            pub fn fssel_2(self) -> &'a mut W {
                self.variant(FSSELW::FSSEL_2)
            }
            #[doc = "Flash clock select: 3 - SMCLK"]
            #[inline(always)]
            pub fn fssel_3(self) -> &'a mut W {
                self.variant(FSSELW::FSSEL_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
            #[doc = "Bit 0 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
            #[inline(always)]
            pub fn fn0(&self) -> FN0R {
                FN0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 1 - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
            #[inline(always)]
            pub fn fn1(&self) -> FN1R {
                FN1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 2 - FN2"]
            #[inline(always)]
            pub fn fn2(&self) -> FN2R {
                FN2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 3 - FN3"]
            #[inline(always)]
            pub fn fn3(&self) -> FN3R {
                FN3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 4 - FN4"]
            #[inline(always)]
            pub fn fn4(&self) -> FN4R {
                FN4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 5 - FN5"]
            #[inline(always)]
            pub fn fn5(&self) -> FN5R {
                FN5R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
            #[inline(always)]
            pub fn fssel(&self) -> FSSELR {
                FSSELR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Divide Flash clock by 1 to 64 using FN0 to FN5 according to:"]
            #[inline(always)]
            pub fn fn0(&mut self) -> _FN0W {
                _FN0W { w: self }
            }
            #[doc = "Bit 1 - 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1"]
            #[inline(always)]
            pub fn fn1(&mut self) -> _FN1W {
                _FN1W { w: self }
            }
            #[doc = "Bit 2 - FN2"]
            #[inline(always)]
            pub fn fn2(&mut self) -> _FN2W {
                _FN2W { w: self }
            }
            #[doc = "Bit 3 - FN3"]
            #[inline(always)]
            pub fn fn3(&mut self) -> _FN3W {
                _FN3W { w: self }
            }
            #[doc = "Bit 4 - FN4"]
            #[inline(always)]
            pub fn fn4(&mut self) -> _FN4W {
                _FN4W { w: self }
            }
            #[doc = "Bit 5 - FN5"]
            #[inline(always)]
            pub fn fn5(&mut self) -> _FN5W {
                _FN5W { w: self }
            }
            #[doc = "Bits 6:7 - Flash clock select 0 */ /* to distinguish from USART SSELx"]
            #[inline(always)]
            pub fn fssel(&mut self) -> _FSSELW {
                _FSSELW { w: self }
            }
        }
    }
    #[doc = "FLASH Control 3"]
    pub struct FCTL3 {
        register: VolatileCell<u16>,
    }
    #[doc = "FLASH Control 3"]
    pub mod fctl3 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::FCTL3 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `BUSY`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum BUSYR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl BUSYR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    BUSYR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> BUSYR {
                match value {
                    i => BUSYR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `KEYV`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum KEYVR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl KEYVR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    KEYVR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> KEYVR {
                match value {
                    i => KEYVR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `ACCVIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum ACCVIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl ACCVIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    ACCVIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> ACCVIFGR {
                match value {
                    i => ACCVIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `WAIT`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WAITR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WAITR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WAITR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WAITR {
                match value {
                    i => WAITR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `LOCK`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum LOCKR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl LOCKR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    LOCKR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> LOCKR {
                match value {
                    i => LOCKR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `EMEX`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum EMEXR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl EMEXR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    EMEXR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> EMEXR {
                match value {
                    i => EMEXR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `LOCKA`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum LOCKAR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl LOCKAR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    LOCKAR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> LOCKAR {
                match value {
                    i => LOCKAR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `FAIL`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FAILR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl FAILR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    FAILR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> FAILR {
                match value {
                    i => FAILR::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `BUSY`"]
        pub enum BUSYW { }
        impl BUSYW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _BUSYW<'a> {
            w: &'a mut W,
        }
        impl<'a> _BUSYW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: BUSYW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `KEYV`"]
        pub enum KEYVW { }
        impl KEYVW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _KEYVW<'a> {
            w: &'a mut W,
        }
        impl<'a> _KEYVW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: KEYVW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `ACCVIFG`"]
        pub enum ACCVIFGW { }
        impl ACCVIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _ACCVIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _ACCVIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: ACCVIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `WAIT`"]
        pub enum WAITW { }
        impl WAITW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WAITW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WAITW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WAITW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `LOCK`"]
        pub enum LOCKW { }
        impl LOCKW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _LOCKW<'a> {
            w: &'a mut W,
        }
        impl<'a> _LOCKW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: LOCKW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `EMEX`"]
        pub enum EMEXW { }
        impl EMEXW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _EMEXW<'a> {
            w: &'a mut W,
        }
        impl<'a> _EMEXW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: EMEXW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `LOCKA`"]
        pub enum LOCKAW { }
        impl LOCKAW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _LOCKAW<'a> {
            w: &'a mut W,
        }
        impl<'a> _LOCKAW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: LOCKAW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `FAIL`"]
        pub enum FAILW { }
        impl FAILW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _FAILW<'a> {
            w: &'a mut W,
        }
        impl<'a> _FAILW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FAILW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
            #[doc = "Bit 0 - Flash busy: 1"]
            #[inline(always)]
            pub fn busy(&self) -> BUSYR {
                BUSYR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 1 - Flash Key violation flag"]
            #[inline(always)]
            pub fn keyv(&self) -> KEYVR {
                KEYVR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 2 - Flash Access violation flag"]
            #[inline(always)]
            pub fn accvifg(&self) -> ACCVIFGR {
                ACCVIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 3 - Wait flag for segment write"]
            #[inline(always)]
            pub fn wait(&self) -> WAITR {
                WAITR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
            #[inline(always)]
            pub fn lock(&self) -> LOCKR {
                LOCKR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 5 - Flash Emergency Exit"]
            #[inline(always)]
            pub fn emex(&self) -> EMEXR {
                EMEXR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
            #[inline(always)]
            pub fn locka(&self) -> LOCKAR {
                LOCKAR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 7 - Last Program or Erase failed"]
            #[inline(always)]
            pub fn fail(&self) -> FAILR {
                FAILR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Flash busy: 1"]
            #[inline(always)]
            pub fn busy(&mut self) -> _BUSYW {
                _BUSYW { w: self }
            }
            #[doc = "Bit 1 - Flash Key violation flag"]
            #[inline(always)]
            pub fn keyv(&mut self) -> _KEYVW {
                _KEYVW { w: self }
            }
            #[doc = "Bit 2 - Flash Access violation flag"]
            #[inline(always)]
            pub fn accvifg(&mut self) -> _ACCVIFGW {
                _ACCVIFGW { w: self }
            }
            #[doc = "Bit 3 - Wait flag for segment write"]
            #[inline(always)]
            pub fn wait(&mut self) -> _WAITW {
                _WAITW { w: self }
            }
            #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
            #[inline(always)]
            pub fn lock(&mut self) -> _LOCKW {
                _LOCKW { w: self }
            }
            #[doc = "Bit 5 - Flash Emergency Exit"]
            #[inline(always)]
            pub fn emex(&mut self) -> _EMEXW {
                _EMEXW { w: self }
            }
            #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
            #[inline(always)]
            pub fn locka(&mut self) -> _LOCKAW {
                _LOCKAW { w: self }
            }
            #[doc = "Bit 7 - Last Program or Erase failed"]
            #[inline(always)]
            pub fn fail(&mut self) -> _FAILW {
                _FAILW { w: self }
            }
        }
    }
}
#[doc = "Flash"]
pub struct FLASH {
    register_block: flash::RegisterBlock,
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Watchdog Timer"]
pub const WATCHDOG_TIMER: Peripheral<WATCHDOG_TIMER> = unsafe { Peripheral::new(288) };
#[doc = "Watchdog Timer"]
pub mod watchdog_timer {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - Watchdog Timer Control"]
        pub wdtctl: WDTCTL,
    }
    #[doc = "Watchdog Timer Control"]
    pub struct WDTCTL {
        register: VolatileCell<u16>,
    }
    #[doc = "Watchdog Timer Control"]
    pub mod wdtctl {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::WDTCTL {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `WDTIS0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTIS0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTIS0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTIS0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTIS0R {
                match value {
                    i => WDTIS0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `WDTIS1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTIS1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTIS1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTIS1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTIS1R {
                match value {
                    i => WDTIS1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `WDTSSEL`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTSSELR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTSSELR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTSSELR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTSSELR {
                match value {
                    i => WDTSSELR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `WDTCNTCL`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTCNTCLR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTCNTCLR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTCNTCLR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTCNTCLR {
                match value {
                    i => WDTCNTCLR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `WDTTMSEL`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTTMSELR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTTMSELR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTTMSELR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTTMSELR {
                match value {
                    i => WDTTMSELR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `WDTNMI`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTNMIR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTNMIR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTNMIR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTNMIR {
                match value {
                    i => WDTNMIR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `WDTNMIES`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTNMIESR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTNMIESR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTNMIESR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTNMIESR {
                match value {
                    i => WDTNMIESR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `WDTHOLD`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum WDTHOLDR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl WDTHOLDR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    WDTHOLDR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> WDTHOLDR {
                match value {
                    i => WDTHOLDR::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `WDTIS0`"]
        pub enum WDTIS0W { }
        impl WDTIS0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTIS0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTIS0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTIS0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `WDTIS1`"]
        pub enum WDTIS1W { }
        impl WDTIS1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTIS1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTIS1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTIS1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `WDTSSEL`"]
        pub enum WDTSSELW { }
        impl WDTSSELW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTSSELW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTSSELW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTSSELW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `WDTCNTCL`"]
        pub enum WDTCNTCLW { }
        impl WDTCNTCLW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTCNTCLW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTCNTCLW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTCNTCLW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `WDTTMSEL`"]
        pub enum WDTTMSELW { }
        impl WDTTMSELW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTTMSELW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTTMSELW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTTMSELW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `WDTNMI`"]
        pub enum WDTNMIW { }
        impl WDTNMIW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTNMIW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTNMIW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTNMIW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `WDTNMIES`"]
        pub enum WDTNMIESW { }
        impl WDTNMIESW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTNMIESW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTNMIESW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTNMIESW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `WDTHOLD`"]
        pub enum WDTHOLDW { }
        impl WDTHOLDW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _WDTHOLDW<'a> {
            w: &'a mut W,
        }
        impl<'a> _WDTHOLDW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: WDTHOLDW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
            #[doc = "Bit 0 - WDTIS0"]
            #[inline(always)]
            pub fn wdtis0(&self) -> WDTIS0R {
                WDTIS0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 1 - WDTIS1"]
            #[inline(always)]
            pub fn wdtis1(&self) -> WDTIS1R {
                WDTIS1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 2 - WDTSSEL"]
            #[inline(always)]
            pub fn wdtssel(&self) -> WDTSSELR {
                WDTSSELR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 3 - WDTCNTCL"]
            #[inline(always)]
            pub fn wdtcntcl(&self) -> WDTCNTCLR {
                WDTCNTCLR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 4 - WDTTMSEL"]
            #[inline(always)]
            pub fn wdttmsel(&self) -> WDTTMSELR {
                WDTTMSELR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 5 - WDTNMI"]
            #[inline(always)]
            pub fn wdtnmi(&self) -> WDTNMIR {
                WDTNMIR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 6 - WDTNMIES"]
            #[inline(always)]
            pub fn wdtnmies(&self) -> WDTNMIESR {
                WDTNMIESR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 7 - WDTHOLD"]
            #[inline(always)]
            pub fn wdthold(&self) -> WDTHOLDR {
                WDTHOLDR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - WDTIS0"]
            #[inline(always)]
            pub fn wdtis0(&mut self) -> _WDTIS0W {
                _WDTIS0W { w: self }
            }
            #[doc = "Bit 1 - WDTIS1"]
            #[inline(always)]
            pub fn wdtis1(&mut self) -> _WDTIS1W {
                _WDTIS1W { w: self }
            }
            #[doc = "Bit 2 - WDTSSEL"]
            #[inline(always)]
            pub fn wdtssel(&mut self) -> _WDTSSELW {
                _WDTSSELW { w: self }
            }
            #[doc = "Bit 3 - WDTCNTCL"]
            #[inline(always)]
            pub fn wdtcntcl(&mut self) -> _WDTCNTCLW {
                _WDTCNTCLW { w: self }
            }
            #[doc = "Bit 4 - WDTTMSEL"]
            #[inline(always)]
            pub fn wdttmsel(&mut self) -> _WDTTMSELW {
                _WDTTMSELW { w: self }
            }
            #[doc = "Bit 5 - WDTNMI"]
            #[inline(always)]
            pub fn wdtnmi(&mut self) -> _WDTNMIW {
                _WDTNMIW { w: self }
            }
            #[doc = "Bit 6 - WDTNMIES"]
            #[inline(always)]
            pub fn wdtnmies(&mut self) -> _WDTNMIESW {
                _WDTNMIESW { w: self }
            }
            #[doc = "Bit 7 - WDTHOLD"]
            #[inline(always)]
            pub fn wdthold(&mut self) -> _WDTHOLDW {
                _WDTHOLDW { w: self }
            }
        }
    }
}
#[doc = "Watchdog Timer"]
pub struct WATCHDOG_TIMER {
    register_block: watchdog_timer::RegisterBlock,
}
impl Deref for WATCHDOG_TIMER {
    type Target = watchdog_timer::RegisterBlock;
    fn deref(&self) -> &watchdog_timer::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Clock"]
pub const SYSTEM_CLOCK: Peripheral<SYSTEM_CLOCK> = unsafe { Peripheral::new(82) };
#[doc = "System Clock"]
pub mod system_clock {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        _reserved0: [u8; 1usize],
        #[doc = "0x01 - Basic Clock System Control 3"]
        pub bcsctl3: BCSCTL3,
        _reserved1: [u8; 2usize],
        #[doc = "0x04 - DCO Clock Frequency Control"]
        pub dcoctl: DCOCTL,
        #[doc = "0x05 - Basic Clock System Control 1"]
        pub bcsctl1: BCSCTL1,
        #[doc = "0x06 - Basic Clock System Control 2"]
        pub bcsctl2: BCSCTL2,
    }
    #[doc = "Basic Clock System Control 3"]
    pub struct BCSCTL3 {
        register: VolatileCell<u8>,
    }
    #[doc = "Basic Clock System Control 3"]
    pub mod bcsctl3 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::BCSCTL3 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `LFXT1OF`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum LFXT1OFR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl LFXT1OFR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    LFXT1OFR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> LFXT1OFR {
                match value {
                    i => LFXT1OFR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `XT2OF`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum XT2OFR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl XT2OFR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    XT2OFR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> XT2OFR {
                match value {
                    i => XT2OFR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `XCAP`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum XCAPR {
            #[doc = "XIN/XOUT Cap : 0 pF"]
            XCAP_0,
            #[doc = "XIN/XOUT Cap : 6 pF"]
            XCAP_1,
            #[doc = "XIN/XOUT Cap : 10 pF"]
            XCAP_2,
            #[doc = "XIN/XOUT Cap : 12.5 pF"]
            XCAP_3,
        }
        impl XCAPR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    XCAPR::XCAP_0 => 0,
                    XCAPR::XCAP_1 => 1,
                    XCAPR::XCAP_2 => 2,
                    XCAPR::XCAP_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> XCAPR {
                match value {
                    0 => XCAPR::XCAP_0,
                    1 => XCAPR::XCAP_1,
                    2 => XCAPR::XCAP_2,
                    3 => XCAPR::XCAP_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `XCAP_0`"]
            #[inline(always)]
            pub fn is_xcap_0(&self) -> bool {
                *self == XCAPR::XCAP_0
            }
            #[doc = "Checks if the value of the field is `XCAP_1`"]
            #[inline(always)]
            pub fn is_xcap_1(&self) -> bool {
                *self == XCAPR::XCAP_1
            }
            #[doc = "Checks if the value of the field is `XCAP_2`"]
            #[inline(always)]
            pub fn is_xcap_2(&self) -> bool {
                *self == XCAPR::XCAP_2
            }
            #[doc = "Checks if the value of the field is `XCAP_3`"]
            #[inline(always)]
            pub fn is_xcap_3(&self) -> bool {
                *self == XCAPR::XCAP_3
            }
        }
        #[doc = "Possible values of the field `LFXT1S`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum LFXT1SR {
            #[doc = "Mode 0 for LFXT1 : Normal operation"]
            LFXT1S_0,
            #[doc = "Mode 1 for LFXT1 : Reserved"]
            LFXT1S_1,
            #[doc = "Mode 2 for LFXT1 : VLO"]
            LFXT1S_2,
            #[doc = "Mode 3 for LFXT1 : Digital input signal"]
            LFXT1S_3,
        }
        impl LFXT1SR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    LFXT1SR::LFXT1S_0 => 0,
                    LFXT1SR::LFXT1S_1 => 1,
                    LFXT1SR::LFXT1S_2 => 2,
                    LFXT1SR::LFXT1S_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> LFXT1SR {
                match value {
                    0 => LFXT1SR::LFXT1S_0,
                    1 => LFXT1SR::LFXT1S_1,
                    2 => LFXT1SR::LFXT1S_2,
                    3 => LFXT1SR::LFXT1S_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `LFXT1S_0`"]
            #[inline(always)]
            pub fn is_lfxt1s_0(&self) -> bool {
                *self == LFXT1SR::LFXT1S_0
            }
            #[doc = "Checks if the value of the field is `LFXT1S_1`"]
            #[inline(always)]
            pub fn is_lfxt1s_1(&self) -> bool {
                *self == LFXT1SR::LFXT1S_1
            }
            #[doc = "Checks if the value of the field is `LFXT1S_2`"]
            #[inline(always)]
            pub fn is_lfxt1s_2(&self) -> bool {
                *self == LFXT1SR::LFXT1S_2
            }
            #[doc = "Checks if the value of the field is `LFXT1S_3`"]
            #[inline(always)]
            pub fn is_lfxt1s_3(&self) -> bool {
                *self == LFXT1SR::LFXT1S_3
            }
        }
        #[doc = "Possible values of the field `XT2S`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum XT2SR {
            #[doc = "Mode 0 for XT2 : 0.4 - 1 MHz"]
            XT2S_0,
            #[doc = "Mode 1 for XT2 : 1 - 4 MHz"]
            XT2S_1,
            #[doc = "Mode 2 for XT2 : 2 - 16 MHz"]
            XT2S_2,
            #[doc = "Mode 3 for XT2 : Digital input signal"]
            XT2S_3,
        }
        impl XT2SR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    XT2SR::XT2S_0 => 0,
                    XT2SR::XT2S_1 => 1,
                    XT2SR::XT2S_2 => 2,
                    XT2SR::XT2S_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> XT2SR {
                match value {
                    0 => XT2SR::XT2S_0,
                    1 => XT2SR::XT2S_1,
                    2 => XT2SR::XT2S_2,
                    3 => XT2SR::XT2S_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `XT2S_0`"]
            #[inline(always)]
            pub fn is_xt2s_0(&self) -> bool {
                *self == XT2SR::XT2S_0
            }
            #[doc = "Checks if the value of the field is `XT2S_1`"]
            #[inline(always)]
            pub fn is_xt2s_1(&self) -> bool {
                *self == XT2SR::XT2S_1
            }
            #[doc = "Checks if the value of the field is `XT2S_2`"]
            #[inline(always)]
            pub fn is_xt2s_2(&self) -> bool {
                *self == XT2SR::XT2S_2
            }
            #[doc = "Checks if the value of the field is `XT2S_3`"]
            #[inline(always)]
            pub fn is_xt2s_3(&self) -> bool {
                *self == XT2SR::XT2S_3
            }
        }
        #[doc = "Values that can be written to the field `LFXT1OF`"]
        pub enum LFXT1OFW { }
        impl LFXT1OFW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _LFXT1OFW<'a> {
            w: &'a mut W,
        }
        impl<'a> _LFXT1OFW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: LFXT1OFW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `XT2OF`"]
        pub enum XT2OFW { }
        impl XT2OFW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _XT2OFW<'a> {
            w: &'a mut W,
        }
        impl<'a> _XT2OFW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: XT2OFW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `XCAP`"]
        pub enum XCAPW {
            #[doc = "XIN/XOUT Cap : 0 pF"]
            XCAP_0,
            #[doc = "XIN/XOUT Cap : 6 pF"]
            XCAP_1,
            #[doc = "XIN/XOUT Cap : 10 pF"]
            XCAP_2,
            #[doc = "XIN/XOUT Cap : 12.5 pF"]
            XCAP_3,
        }
        impl XCAPW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    XCAPW::XCAP_0 => 0,
                    XCAPW::XCAP_1 => 1,
                    XCAPW::XCAP_2 => 2,
                    XCAPW::XCAP_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _XCAPW<'a> {
            w: &'a mut W,
        }
        impl<'a> _XCAPW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: XCAPW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "XIN/XOUT Cap : 0 pF"]
            #[inline(always)]
            pub fn xcap_0(self) -> &'a mut W {
                self.variant(XCAPW::XCAP_0)
            }
            #[doc = "XIN/XOUT Cap : 6 pF"]
            #[inline(always)]
            pub fn xcap_1(self) -> &'a mut W {
                self.variant(XCAPW::XCAP_1)
            }
            #[doc = "XIN/XOUT Cap : 10 pF"]
            #[inline(always)]
            pub fn xcap_2(self) -> &'a mut W {
                self.variant(XCAPW::XCAP_2)
            }
            #[doc = "XIN/XOUT Cap : 12.5 pF"]
            #[inline(always)]
            pub fn xcap_3(self) -> &'a mut W {
                self.variant(XCAPW::XCAP_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `LFXT1S`"]
        pub enum LFXT1SW {
            #[doc = "Mode 0 for LFXT1 : Normal operation"]
            LFXT1S_0,
            #[doc = "Mode 1 for LFXT1 : Reserved"]
            LFXT1S_1,
            #[doc = "Mode 2 for LFXT1 : VLO"]
            LFXT1S_2,
            #[doc = "Mode 3 for LFXT1 : Digital input signal"]
            LFXT1S_3,
        }
        impl LFXT1SW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    LFXT1SW::LFXT1S_0 => 0,
                    LFXT1SW::LFXT1S_1 => 1,
                    LFXT1SW::LFXT1S_2 => 2,
                    LFXT1SW::LFXT1S_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _LFXT1SW<'a> {
            w: &'a mut W,
        }
        impl<'a> _LFXT1SW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: LFXT1SW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Mode 0 for LFXT1 : Normal operation"]
            #[inline(always)]
            pub fn lfxt1s_0(self) -> &'a mut W {
                self.variant(LFXT1SW::LFXT1S_0)
            }
            #[doc = "Mode 1 for LFXT1 : Reserved"]
            #[inline(always)]
            pub fn lfxt1s_1(self) -> &'a mut W {
                self.variant(LFXT1SW::LFXT1S_1)
            }
            #[doc = "Mode 2 for LFXT1 : VLO"]
            #[inline(always)]
            pub fn lfxt1s_2(self) -> &'a mut W {
                self.variant(LFXT1SW::LFXT1S_2)
            }
            #[doc = "Mode 3 for LFXT1 : Digital input signal"]
            #[inline(always)]
            pub fn lfxt1s_3(self) -> &'a mut W {
                self.variant(LFXT1SW::LFXT1S_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `XT2S`"]
        pub enum XT2SW {
            #[doc = "Mode 0 for XT2 : 0.4 - 1 MHz"]
            XT2S_0,
            #[doc = "Mode 1 for XT2 : 1 - 4 MHz"]
            XT2S_1,
            #[doc = "Mode 2 for XT2 : 2 - 16 MHz"]
            XT2S_2,
            #[doc = "Mode 3 for XT2 : Digital input signal"]
            XT2S_3,
        }
        impl XT2SW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    XT2SW::XT2S_0 => 0,
                    XT2SW::XT2S_1 => 1,
                    XT2SW::XT2S_2 => 2,
                    XT2SW::XT2S_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _XT2SW<'a> {
            w: &'a mut W,
        }
        impl<'a> _XT2SW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: XT2SW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Mode 0 for XT2 : 0.4 - 1 MHz"]
            #[inline(always)]
            pub fn xt2s_0(self) -> &'a mut W {
                self.variant(XT2SW::XT2S_0)
            }
            #[doc = "Mode 1 for XT2 : 1 - 4 MHz"]
            #[inline(always)]
            pub fn xt2s_1(self) -> &'a mut W {
                self.variant(XT2SW::XT2S_1)
            }
            #[doc = "Mode 2 for XT2 : 2 - 16 MHz"]
            #[inline(always)]
            pub fn xt2s_2(self) -> &'a mut W {
                self.variant(XT2SW::XT2S_2)
            }
            #[doc = "Mode 3 for XT2 : Digital input signal"]
            #[inline(always)]
            pub fn xt2s_3(self) -> &'a mut W {
                self.variant(XT2SW::XT2S_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - Low/high Frequency Oscillator Fault Flag"]
            #[inline(always)]
            pub fn lfxt1of(&self) -> LFXT1OFR {
                LFXT1OFR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - High frequency oscillator 2 fault flag"]
            #[inline(always)]
            pub fn xt2of(&self) -> XT2OFR {
                XT2OFR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bits 2:3 - XIN/XOUT Cap 0"]
            #[inline(always)]
            pub fn xcap(&self) -> XCAPR {
                XCAPR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) as u8
                })
            }
            #[doc = "Bits 4:5 - Mode 0 for LFXT1 (XTS = 0)"]
            #[inline(always)]
            pub fn lfxt1s(&self) -> LFXT1SR {
                LFXT1SR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) as u8
                })
            }
            #[doc = "Bits 6:7 - Mode 0 for XT2"]
            #[inline(always)]
            pub fn xt2s(&self) -> XT2SR {
                XT2SR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) as u8
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Low/high Frequency Oscillator Fault Flag"]
            #[inline(always)]
            pub fn lfxt1of(&mut self) -> _LFXT1OFW {
                _LFXT1OFW { w: self }
            }
            #[doc = "Bit 1 - High frequency oscillator 2 fault flag"]
            #[inline(always)]
            pub fn xt2of(&mut self) -> _XT2OFW {
                _XT2OFW { w: self }
            }
            #[doc = "Bits 2:3 - XIN/XOUT Cap 0"]
            #[inline(always)]
            pub fn xcap(&mut self) -> _XCAPW {
                _XCAPW { w: self }
            }
            #[doc = "Bits 4:5 - Mode 0 for LFXT1 (XTS = 0)"]
            #[inline(always)]
            pub fn lfxt1s(&mut self) -> _LFXT1SW {
                _LFXT1SW { w: self }
            }
            #[doc = "Bits 6:7 - Mode 0 for XT2"]
            #[inline(always)]
            pub fn xt2s(&mut self) -> _XT2SW {
                _XT2SW { w: self }
            }
        }
    }
    #[doc = "DCO Clock Frequency Control"]
    pub struct DCOCTL {
        register: VolatileCell<u8>,
    }
    #[doc = "DCO Clock Frequency Control"]
    pub mod dcoctl {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::DCOCTL {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `MOD0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum MOD0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl MOD0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    MOD0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> MOD0R {
                match value {
                    i => MOD0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `MOD1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum MOD1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl MOD1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    MOD1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> MOD1R {
                match value {
                    i => MOD1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `MOD2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum MOD2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl MOD2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    MOD2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> MOD2R {
                match value {
                    i => MOD2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `MOD3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum MOD3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl MOD3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    MOD3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> MOD3R {
                match value {
                    i => MOD3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `MOD4`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum MOD4R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl MOD4R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    MOD4R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> MOD4R {
                match value {
                    i => MOD4R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `DCO0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum DCO0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl DCO0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    DCO0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> DCO0R {
                match value {
                    i => DCO0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `DCO1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum DCO1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl DCO1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    DCO1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> DCO1R {
                match value {
                    i => DCO1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `DCO2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum DCO2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl DCO2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    DCO2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> DCO2R {
                match value {
                    i => DCO2R::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `MOD0`"]
        pub enum MOD0W { }
        impl MOD0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _MOD0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _MOD0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: MOD0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `MOD1`"]
        pub enum MOD1W { }
        impl MOD1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _MOD1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _MOD1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: MOD1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `MOD2`"]
        pub enum MOD2W { }
        impl MOD2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _MOD2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _MOD2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: MOD2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `MOD3`"]
        pub enum MOD3W { }
        impl MOD3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _MOD3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _MOD3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: MOD3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `MOD4`"]
        pub enum MOD4W { }
        impl MOD4W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _MOD4W<'a> {
            w: &'a mut W,
        }
        impl<'a> _MOD4W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: MOD4W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `DCO0`"]
        pub enum DCO0W { }
        impl DCO0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _DCO0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _DCO0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: DCO0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `DCO1`"]
        pub enum DCO1W { }
        impl DCO1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _DCO1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _DCO1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: DCO1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `DCO2`"]
        pub enum DCO2W { }
        impl DCO2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _DCO2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _DCO2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: DCO2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - Modulation Bit 0"]
            #[inline(always)]
            pub fn mod0(&self) -> MOD0R {
                MOD0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - Modulation Bit 1"]
            #[inline(always)]
            pub fn mod1(&self) -> MOD1R {
                MOD1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - Modulation Bit 2"]
            #[inline(always)]
            pub fn mod2(&self) -> MOD2R {
                MOD2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - Modulation Bit 3"]
            #[inline(always)]
            pub fn mod3(&self) -> MOD3R {
                MOD3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 4 - Modulation Bit 4"]
            #[inline(always)]
            pub fn mod4(&self) -> MOD4R {
                MOD4R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 5 - DCO Select Bit 0"]
            #[inline(always)]
            pub fn dco0(&self) -> DCO0R {
                DCO0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 6 - DCO Select Bit 1"]
            #[inline(always)]
            pub fn dco1(&self) -> DCO1R {
                DCO1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - DCO Select Bit 2"]
            #[inline(always)]
            pub fn dco2(&self) -> DCO2R {
                DCO2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Modulation Bit 0"]
            #[inline(always)]
            pub fn mod0(&mut self) -> _MOD0W {
                _MOD0W { w: self }
            }
            #[doc = "Bit 1 - Modulation Bit 1"]
            #[inline(always)]
            pub fn mod1(&mut self) -> _MOD1W {
                _MOD1W { w: self }
            }
            #[doc = "Bit 2 - Modulation Bit 2"]
            #[inline(always)]
            pub fn mod2(&mut self) -> _MOD2W {
                _MOD2W { w: self }
            }
            #[doc = "Bit 3 - Modulation Bit 3"]
            #[inline(always)]
            pub fn mod3(&mut self) -> _MOD3W {
                _MOD3W { w: self }
            }
            #[doc = "Bit 4 - Modulation Bit 4"]
            #[inline(always)]
            pub fn mod4(&mut self) -> _MOD4W {
                _MOD4W { w: self }
            }
            #[doc = "Bit 5 - DCO Select Bit 0"]
            #[inline(always)]
            pub fn dco0(&mut self) -> _DCO0W {
                _DCO0W { w: self }
            }
            #[doc = "Bit 6 - DCO Select Bit 1"]
            #[inline(always)]
            pub fn dco1(&mut self) -> _DCO1W {
                _DCO1W { w: self }
            }
            #[doc = "Bit 7 - DCO Select Bit 2"]
            #[inline(always)]
            pub fn dco2(&mut self) -> _DCO2W {
                _DCO2W { w: self }
            }
        }
    }
    #[doc = "Basic Clock System Control 1"]
    pub struct BCSCTL1 {
        register: VolatileCell<u8>,
    }
    #[doc = "Basic Clock System Control 1"]
    pub mod bcsctl1 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::BCSCTL1 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `RSEL0`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum RSEL0R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl RSEL0R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    RSEL0R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> RSEL0R {
                match value {
                    i => RSEL0R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `RSEL1`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum RSEL1R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl RSEL1R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    RSEL1R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> RSEL1R {
                match value {
                    i => RSEL1R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `RSEL2`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum RSEL2R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl RSEL2R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    RSEL2R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> RSEL2R {
                match value {
                    i => RSEL2R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `RSEL3`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum RSEL3R {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl RSEL3R {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    RSEL3R::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> RSEL3R {
                match value {
                    i => RSEL3R::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `DIVA`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum DIVAR {
            #[doc = "ACLK Divider 0: /1"]
            DIVA_0,
            #[doc = "ACLK Divider 1: /2"]
            DIVA_1,
            #[doc = "ACLK Divider 2: /4"]
            DIVA_2,
            #[doc = "ACLK Divider 3: /8"]
            DIVA_3,
        }
        impl DIVAR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    DIVAR::DIVA_0 => 0,
                    DIVAR::DIVA_1 => 1,
                    DIVAR::DIVA_2 => 2,
                    DIVAR::DIVA_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> DIVAR {
                match value {
                    0 => DIVAR::DIVA_0,
                    1 => DIVAR::DIVA_1,
                    2 => DIVAR::DIVA_2,
                    3 => DIVAR::DIVA_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `DIVA_0`"]
            #[inline(always)]
            pub fn is_diva_0(&self) -> bool {
                *self == DIVAR::DIVA_0
            }
            #[doc = "Checks if the value of the field is `DIVA_1`"]
            #[inline(always)]
            pub fn is_diva_1(&self) -> bool {
                *self == DIVAR::DIVA_1
            }
            #[doc = "Checks if the value of the field is `DIVA_2`"]
            #[inline(always)]
            pub fn is_diva_2(&self) -> bool {
                *self == DIVAR::DIVA_2
            }
            #[doc = "Checks if the value of the field is `DIVA_3`"]
            #[inline(always)]
            pub fn is_diva_3(&self) -> bool {
                *self == DIVAR::DIVA_3
            }
        }
        #[doc = "Possible values of the field `XTS`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum XTSR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl XTSR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    XTSR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> XTSR {
                match value {
                    i => XTSR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `XT2OFF`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum XT2OFFR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl XT2OFFR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    XT2OFFR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> XT2OFFR {
                match value {
                    i => XT2OFFR::_Reserved(i),
                }
            }
        }
        #[doc = "Values that can be written to the field `RSEL0`"]
        pub enum RSEL0W { }
        impl RSEL0W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _RSEL0W<'a> {
            w: &'a mut W,
        }
        impl<'a> _RSEL0W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: RSEL0W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `RSEL1`"]
        pub enum RSEL1W { }
        impl RSEL1W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _RSEL1W<'a> {
            w: &'a mut W,
        }
        impl<'a> _RSEL1W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: RSEL1W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `RSEL2`"]
        pub enum RSEL2W { }
        impl RSEL2W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _RSEL2W<'a> {
            w: &'a mut W,
        }
        impl<'a> _RSEL2W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: RSEL2W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `RSEL3`"]
        pub enum RSEL3W { }
        impl RSEL3W {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _RSEL3W<'a> {
            w: &'a mut W,
        }
        impl<'a> _RSEL3W<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: RSEL3W) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `DIVA`"]
        pub enum DIVAW {
            #[doc = "ACLK Divider 0: /1"]
            DIVA_0,
            #[doc = "ACLK Divider 1: /2"]
            DIVA_1,
            #[doc = "ACLK Divider 2: /4"]
            DIVA_2,
            #[doc = "ACLK Divider 3: /8"]
            DIVA_3,
        }
        impl DIVAW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    DIVAW::DIVA_0 => 0,
                    DIVAW::DIVA_1 => 1,
                    DIVAW::DIVA_2 => 2,
                    DIVAW::DIVA_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _DIVAW<'a> {
            w: &'a mut W,
        }
        impl<'a> _DIVAW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: DIVAW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "ACLK Divider 0: /1"]
            #[inline(always)]
            pub fn diva_0(self) -> &'a mut W {
                self.variant(DIVAW::DIVA_0)
            }
            #[doc = "ACLK Divider 1: /2"]
            #[inline(always)]
            pub fn diva_1(self) -> &'a mut W {
                self.variant(DIVAW::DIVA_1)
            }
            #[doc = "ACLK Divider 2: /4"]
            #[inline(always)]
            pub fn diva_2(self) -> &'a mut W {
                self.variant(DIVAW::DIVA_2)
            }
            #[doc = "ACLK Divider 3: /8"]
            #[inline(always)]
            pub fn diva_3(self) -> &'a mut W {
                self.variant(DIVAW::DIVA_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `XTS`"]
        pub enum XTSW { }
        impl XTSW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _XTSW<'a> {
            w: &'a mut W,
        }
        impl<'a> _XTSW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: XTSW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `XT2OFF`"]
        pub enum XT2OFFW { }
        impl XT2OFFW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _XT2OFFW<'a> {
            w: &'a mut W,
        }
        impl<'a> _XT2OFFW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: XT2OFFW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 7;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bit 0 - Range Select Bit 0"]
            #[inline(always)]
            pub fn rsel0(&self) -> RSEL0R {
                RSEL0R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 1 - Range Select Bit 1"]
            #[inline(always)]
            pub fn rsel1(&self) -> RSEL1R {
                RSEL1R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 2 - Range Select Bit 2"]
            #[inline(always)]
            pub fn rsel2(&self) -> RSEL2R {
                RSEL2R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 3 - Range Select Bit 3"]
            #[inline(always)]
            pub fn rsel3(&self) -> RSEL3R {
                RSEL3R::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bits 4:5 - ACLK Divider 0"]
            #[inline(always)]
            pub fn diva(&self) -> DIVAR {
                DIVAR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) as u8
                })
            }
            #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
            #[inline(always)]
            pub fn xts(&self) -> XTSR {
                XTSR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bit 7 - Enable XT2CLK"]
            #[inline(always)]
            pub fn xt2off(&self) -> XT2OFFR {
                XT2OFFR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 7;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Range Select Bit 0"]
            #[inline(always)]
            pub fn rsel0(&mut self) -> _RSEL0W {
                _RSEL0W { w: self }
            }
            #[doc = "Bit 1 - Range Select Bit 1"]
            #[inline(always)]
            pub fn rsel1(&mut self) -> _RSEL1W {
                _RSEL1W { w: self }
            }
            #[doc = "Bit 2 - Range Select Bit 2"]
            #[inline(always)]
            pub fn rsel2(&mut self) -> _RSEL2W {
                _RSEL2W { w: self }
            }
            #[doc = "Bit 3 - Range Select Bit 3"]
            #[inline(always)]
            pub fn rsel3(&mut self) -> _RSEL3W {
                _RSEL3W { w: self }
            }
            #[doc = "Bits 4:5 - ACLK Divider 0"]
            #[inline(always)]
            pub fn diva(&mut self) -> _DIVAW {
                _DIVAW { w: self }
            }
            #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
            #[inline(always)]
            pub fn xts(&mut self) -> _XTSW {
                _XTSW { w: self }
            }
            #[doc = "Bit 7 - Enable XT2CLK"]
            #[inline(always)]
            pub fn xt2off(&mut self) -> _XT2OFFW {
                _XT2OFFW { w: self }
            }
        }
    }
    #[doc = "Basic Clock System Control 2"]
    pub struct BCSCTL2 {
        register: VolatileCell<u8>,
    }
    #[doc = "Basic Clock System Control 2"]
    pub mod bcsctl2 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::BCSCTL2 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `DIVS`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum DIVSR {
            #[doc = "SMCLK Divider 0: /1"]
            DIVS_0,
            #[doc = "SMCLK Divider 1: /2"]
            DIVS_1,
            #[doc = "SMCLK Divider 2: /4"]
            DIVS_2,
            #[doc = "SMCLK Divider 3: /8"]
            DIVS_3,
        }
        impl DIVSR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    DIVSR::DIVS_0 => 0,
                    DIVSR::DIVS_1 => 1,
                    DIVSR::DIVS_2 => 2,
                    DIVSR::DIVS_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> DIVSR {
                match value {
                    0 => DIVSR::DIVS_0,
                    1 => DIVSR::DIVS_1,
                    2 => DIVSR::DIVS_2,
                    3 => DIVSR::DIVS_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `DIVS_0`"]
            #[inline(always)]
            pub fn is_divs_0(&self) -> bool {
                *self == DIVSR::DIVS_0
            }
            #[doc = "Checks if the value of the field is `DIVS_1`"]
            #[inline(always)]
            pub fn is_divs_1(&self) -> bool {
                *self == DIVSR::DIVS_1
            }
            #[doc = "Checks if the value of the field is `DIVS_2`"]
            #[inline(always)]
            pub fn is_divs_2(&self) -> bool {
                *self == DIVSR::DIVS_2
            }
            #[doc = "Checks if the value of the field is `DIVS_3`"]
            #[inline(always)]
            pub fn is_divs_3(&self) -> bool {
                *self == DIVSR::DIVS_3
            }
        }
        #[doc = "Possible values of the field `SELS`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum SELSR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl SELSR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    SELSR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> SELSR {
                match value {
                    i => SELSR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `DIVM`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum DIVMR {
            #[doc = "MCLK Divider 0: /1"]
            DIVM_0,
            #[doc = "MCLK Divider 1: /2"]
            DIVM_1,
            #[doc = "MCLK Divider 2: /4"]
            DIVM_2,
            #[doc = "MCLK Divider 3: /8"]
            DIVM_3,
        }
        impl DIVMR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    DIVMR::DIVM_0 => 0,
                    DIVMR::DIVM_1 => 1,
                    DIVMR::DIVM_2 => 2,
                    DIVMR::DIVM_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> DIVMR {
                match value {
                    0 => DIVMR::DIVM_0,
                    1 => DIVMR::DIVM_1,
                    2 => DIVMR::DIVM_2,
                    3 => DIVMR::DIVM_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `DIVM_0`"]
            #[inline(always)]
            pub fn is_divm_0(&self) -> bool {
                *self == DIVMR::DIVM_0
            }
            #[doc = "Checks if the value of the field is `DIVM_1`"]
            #[inline(always)]
            pub fn is_divm_1(&self) -> bool {
                *self == DIVMR::DIVM_1
            }
            #[doc = "Checks if the value of the field is `DIVM_2`"]
            #[inline(always)]
            pub fn is_divm_2(&self) -> bool {
                *self == DIVMR::DIVM_2
            }
            #[doc = "Checks if the value of the field is `DIVM_3`"]
            #[inline(always)]
            pub fn is_divm_3(&self) -> bool {
                *self == DIVMR::DIVM_3
            }
        }
        #[doc = "Possible values of the field `SELM`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum SELMR {
            #[doc = "MCLK Source Select 0: DCOCLK"]
            SELM_0,
            #[doc = "MCLK Source Select 1: DCOCLK"]
            SELM_1,
            #[doc = "MCLK Source Select 2: XT2CLK/LFXTCLK"]
            SELM_2,
            #[doc = "MCLK Source Select 3: LFXTCLK"]
            SELM_3,
        }
        impl SELMR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    SELMR::SELM_0 => 0,
                    SELMR::SELM_1 => 1,
                    SELMR::SELM_2 => 2,
                    SELMR::SELM_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> SELMR {
                match value {
                    0 => SELMR::SELM_0,
                    1 => SELMR::SELM_1,
                    2 => SELMR::SELM_2,
                    3 => SELMR::SELM_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `SELM_0`"]
            #[inline(always)]
            pub fn is_selm_0(&self) -> bool {
                *self == SELMR::SELM_0
            }
            #[doc = "Checks if the value of the field is `SELM_1`"]
            #[inline(always)]
            pub fn is_selm_1(&self) -> bool {
                *self == SELMR::SELM_1
            }
            #[doc = "Checks if the value of the field is `SELM_2`"]
            #[inline(always)]
            pub fn is_selm_2(&self) -> bool {
                *self == SELMR::SELM_2
            }
            #[doc = "Checks if the value of the field is `SELM_3`"]
            #[inline(always)]
            pub fn is_selm_3(&self) -> bool {
                *self == SELMR::SELM_3
            }
        }
        #[doc = "Values that can be written to the field `DIVS`"]
        pub enum DIVSW {
            #[doc = "SMCLK Divider 0: /1"]
            DIVS_0,
            #[doc = "SMCLK Divider 1: /2"]
            DIVS_1,
            #[doc = "SMCLK Divider 2: /4"]
            DIVS_2,
            #[doc = "SMCLK Divider 3: /8"]
            DIVS_3,
        }
        impl DIVSW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    DIVSW::DIVS_0 => 0,
                    DIVSW::DIVS_1 => 1,
                    DIVSW::DIVS_2 => 2,
                    DIVSW::DIVS_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _DIVSW<'a> {
            w: &'a mut W,
        }
        impl<'a> _DIVSW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: DIVSW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "SMCLK Divider 0: /1"]
            #[inline(always)]
            pub fn divs_0(self) -> &'a mut W {
                self.variant(DIVSW::DIVS_0)
            }
            #[doc = "SMCLK Divider 1: /2"]
            #[inline(always)]
            pub fn divs_1(self) -> &'a mut W {
                self.variant(DIVSW::DIVS_1)
            }
            #[doc = "SMCLK Divider 2: /4"]
            #[inline(always)]
            pub fn divs_2(self) -> &'a mut W {
                self.variant(DIVSW::DIVS_2)
            }
            #[doc = "SMCLK Divider 3: /8"]
            #[inline(always)]
            pub fn divs_3(self) -> &'a mut W {
                self.variant(DIVSW::DIVS_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `SELS`"]
        pub enum SELSW { }
        impl SELSW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _SELSW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SELSW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: SELSW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `DIVM`"]
        pub enum DIVMW {
            #[doc = "MCLK Divider 0: /1"]
            DIVM_0,
            #[doc = "MCLK Divider 1: /2"]
            DIVM_1,
            #[doc = "MCLK Divider 2: /4"]
            DIVM_2,
            #[doc = "MCLK Divider 3: /8"]
            DIVM_3,
        }
        impl DIVMW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    DIVMW::DIVM_0 => 0,
                    DIVMW::DIVM_1 => 1,
                    DIVMW::DIVM_2 => 2,
                    DIVMW::DIVM_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _DIVMW<'a> {
            w: &'a mut W,
        }
        impl<'a> _DIVMW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: DIVMW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "MCLK Divider 0: /1"]
            #[inline(always)]
            pub fn divm_0(self) -> &'a mut W {
                self.variant(DIVMW::DIVM_0)
            }
            #[doc = "MCLK Divider 1: /2"]
            #[inline(always)]
            pub fn divm_1(self) -> &'a mut W {
                self.variant(DIVMW::DIVM_1)
            }
            #[doc = "MCLK Divider 2: /4"]
            #[inline(always)]
            pub fn divm_2(self) -> &'a mut W {
                self.variant(DIVMW::DIVM_2)
            }
            #[doc = "MCLK Divider 3: /8"]
            #[inline(always)]
            pub fn divm_3(self) -> &'a mut W {
                self.variant(DIVMW::DIVM_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `SELM`"]
        pub enum SELMW {
            #[doc = "MCLK Source Select 0: DCOCLK"]
            SELM_0,
            #[doc = "MCLK Source Select 1: DCOCLK"]
            SELM_1,
            #[doc = "MCLK Source Select 2: XT2CLK/LFXTCLK"]
            SELM_2,
            #[doc = "MCLK Source Select 3: LFXTCLK"]
            SELM_3,
        }
        impl SELMW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    SELMW::SELM_0 => 0,
                    SELMW::SELM_1 => 1,
                    SELMW::SELM_2 => 2,
                    SELMW::SELM_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _SELMW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SELMW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: SELMW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "MCLK Source Select 0: DCOCLK"]
            #[inline(always)]
            pub fn selm_0(self) -> &'a mut W {
                self.variant(SELMW::SELM_0)
            }
            #[doc = "MCLK Source Select 1: DCOCLK"]
            #[inline(always)]
            pub fn selm_1(self) -> &'a mut W {
                self.variant(SELMW::SELM_1)
            }
            #[doc = "MCLK Source Select 2: XT2CLK/LFXTCLK"]
            #[inline(always)]
            pub fn selm_2(self) -> &'a mut W {
                self.variant(SELMW::SELM_2)
            }
            #[doc = "MCLK Source Select 3: LFXTCLK"]
            #[inline(always)]
            pub fn selm_3(self) -> &'a mut W {
                self.variant(SELMW::SELM_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u8) << OFFSET);
                self.w.bits |= ((value & MASK) as u8) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
            #[doc = "Bits 1:2 - SMCLK Divider 0"]
            #[inline(always)]
            pub fn divs(&self) -> DIVSR {
                DIVSR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u8) as u8
                })
            }
            #[doc = "Bit 3 - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
            #[inline(always)]
            pub fn sels(&self) -> SELSR {
                SELSR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u8) != 0
                })
            }
            #[doc = "Bits 4:5 - MCLK Divider 0"]
            #[inline(always)]
            pub fn divm(&self) -> DIVMR {
                DIVMR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u8) as u8
                })
            }
            #[doc = "Bits 6:7 - MCLK Source Select 0"]
            #[inline(always)]
            pub fn selm(&self) -> SELMR {
                SELMR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u8) as u8
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bits 1:2 - SMCLK Divider 0"]
            #[inline(always)]
            pub fn divs(&mut self) -> _DIVSW {
                _DIVSW { w: self }
            }
            #[doc = "Bit 3 - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
            #[inline(always)]
            pub fn sels(&mut self) -> _SELSW {
                _SELSW { w: self }
            }
            #[doc = "Bits 4:5 - MCLK Divider 0"]
            #[inline(always)]
            pub fn divm(&mut self) -> _DIVMW {
                _DIVMW { w: self }
            }
            #[doc = "Bits 6:7 - MCLK Source Select 0"]
            #[inline(always)]
            pub fn selm(&mut self) -> _SELMW {
                _SELMW { w: self }
            }
        }
    }
}
#[doc = "System Clock"]
pub struct SYSTEM_CLOCK {
    register_block: system_clock::RegisterBlock,
}
impl Deref for SYSTEM_CLOCK {
    type Target = system_clock::RegisterBlock;
    fn deref(&self) -> &system_clock::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Timer A2"]
pub const TIMER_A2: Peripheral<TIMER_A2> = unsafe { Peripheral::new(302) };
#[doc = "Timer A2"]
pub mod timer_a2 {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - Timer A Interrupt Vector Word"]
        pub taiv: TAIV,
        _reserved0: [u8; 48usize],
        #[doc = "0x32 - Timer A Control"]
        pub tactl: TACTL,
        #[doc = "0x34 - Timer A Capture/Compare Control 0"]
        pub tacctl0: TACCTL0,
        #[doc = "0x36 - Timer A Capture/Compare Control 1"]
        pub tacctl1: TACCTL1,
        _reserved1: [u8; 10usize],
        #[doc = "0x42 - Timer A Counter Register"]
        pub tar: TAR,
        #[doc = "0x44 - Timer A Capture/Compare 0"]
        pub taccr0: TACCR0,
        #[doc = "0x46 - Timer A Capture/Compare 1"]
        pub taccr1: TACCR1,
    }
    #[doc = "Timer A Interrupt Vector Word"]
    pub struct TAIV {
        register: VolatileCell<u16>,
    }
    #[doc = "Timer A Interrupt Vector Word"]
    pub mod taiv {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::TAIV {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "Timer A Control"]
    pub struct TACTL {
        register: VolatileCell<u16>,
    }
    #[doc = "Timer A Control"]
    pub mod tactl {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::TACTL {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `TAIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum TAIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl TAIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    TAIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> TAIFGR {
                match value {
                    i => TAIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `TAIE`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum TAIER {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl TAIER {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    TAIER::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> TAIER {
                match value {
                    i => TAIER::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `TACLR`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum TACLRR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl TACLRR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    TACLRR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> TACLRR {
                match value {
                    i => TACLRR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `MC`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum MCR {
            #[doc = "Timer A mode control: 0 - Stop"]
            MC_0,
            #[doc = "Timer A mode control: 1 - Up to CCR0"]
            MC_1,
            #[doc = "Timer A mode control: 2 - Continous up"]
            MC_2,
            #[doc = "Timer A mode control: 3 - Up/Down"]
            MC_3,
        }
        impl MCR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    MCR::MC_0 => 0,
                    MCR::MC_1 => 1,
                    MCR::MC_2 => 2,
                    MCR::MC_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> MCR {
                match value {
                    0 => MCR::MC_0,
                    1 => MCR::MC_1,
                    2 => MCR::MC_2,
                    3 => MCR::MC_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `MC_0`"]
            #[inline(always)]
            pub fn is_mc_0(&self) -> bool {
                *self == MCR::MC_0
            }
            #[doc = "Checks if the value of the field is `MC_1`"]
            #[inline(always)]
            pub fn is_mc_1(&self) -> bool {
                *self == MCR::MC_1
            }
            #[doc = "Checks if the value of the field is `MC_2`"]
            #[inline(always)]
            pub fn is_mc_2(&self) -> bool {
                *self == MCR::MC_2
            }
            #[doc = "Checks if the value of the field is `MC_3`"]
            #[inline(always)]
            pub fn is_mc_3(&self) -> bool {
                *self == MCR::MC_3
            }
        }
        #[doc = "Possible values of the field `ID`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum IDR {
            #[doc = "Timer A input divider: 0 - /1"]
            ID_0,
            #[doc = "Timer A input divider: 1 - /2"]
            ID_1,
            #[doc = "Timer A input divider: 2 - /4"]
            ID_2,
            #[doc = "Timer A input divider: 3 - /8"]
            ID_3,
        }
        impl IDR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    IDR::ID_0 => 0,
                    IDR::ID_1 => 1,
                    IDR::ID_2 => 2,
                    IDR::ID_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> IDR {
                match value {
                    0 => IDR::ID_0,
                    1 => IDR::ID_1,
                    2 => IDR::ID_2,
                    3 => IDR::ID_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `ID_0`"]
            #[inline(always)]
            pub fn is_id_0(&self) -> bool {
                *self == IDR::ID_0
            }
            #[doc = "Checks if the value of the field is `ID_1`"]
            #[inline(always)]
            pub fn is_id_1(&self) -> bool {
                *self == IDR::ID_1
            }
            #[doc = "Checks if the value of the field is `ID_2`"]
            #[inline(always)]
            pub fn is_id_2(&self) -> bool {
                *self == IDR::ID_2
            }
            #[doc = "Checks if the value of the field is `ID_3`"]
            #[inline(always)]
            pub fn is_id_3(&self) -> bool {
                *self == IDR::ID_3
            }
        }
        #[doc = "Possible values of the field `TASSEL`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum TASSELR {
            #[doc = "Timer A clock source select: 0 - TACLK"]
            TASSEL_0,
            #[doc = "Timer A clock source select: 1 - ACLK"]
            TASSEL_1,
            #[doc = "Timer A clock source select: 2 - SMCLK"]
            TASSEL_2,
            #[doc = "Timer A clock source select: 3 - INCLK"]
            TASSEL_3,
        }
        impl TASSELR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    TASSELR::TASSEL_0 => 0,
                    TASSELR::TASSEL_1 => 1,
                    TASSELR::TASSEL_2 => 2,
                    TASSELR::TASSEL_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> TASSELR {
                match value {
                    0 => TASSELR::TASSEL_0,
                    1 => TASSELR::TASSEL_1,
                    2 => TASSELR::TASSEL_2,
                    3 => TASSELR::TASSEL_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `TASSEL_0`"]
            #[inline(always)]
            pub fn is_tassel_0(&self) -> bool {
                *self == TASSELR::TASSEL_0
            }
            #[doc = "Checks if the value of the field is `TASSEL_1`"]
            #[inline(always)]
            pub fn is_tassel_1(&self) -> bool {
                *self == TASSELR::TASSEL_1
            }
            #[doc = "Checks if the value of the field is `TASSEL_2`"]
            #[inline(always)]
            pub fn is_tassel_2(&self) -> bool {
                *self == TASSELR::TASSEL_2
            }
            #[doc = "Checks if the value of the field is `TASSEL_3`"]
            #[inline(always)]
            pub fn is_tassel_3(&self) -> bool {
                *self == TASSELR::TASSEL_3
            }
        }
        #[doc = "Values that can be written to the field `TAIFG`"]
        pub enum TAIFGW { }
        impl TAIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _TAIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TAIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: TAIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `TAIE`"]
        pub enum TAIEW { }
        impl TAIEW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _TAIEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TAIEW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: TAIEW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `TACLR`"]
        pub enum TACLRW { }
        impl TACLRW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _TACLRW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TACLRW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: TACLRW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `MC`"]
        pub enum MCW {
            #[doc = "Timer A mode control: 0 - Stop"]
            MC_0,
            #[doc = "Timer A mode control: 1 - Up to CCR0"]
            MC_1,
            #[doc = "Timer A mode control: 2 - Continous up"]
            MC_2,
            #[doc = "Timer A mode control: 3 - Up/Down"]
            MC_3,
        }
        impl MCW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    MCW::MC_0 => 0,
                    MCW::MC_1 => 1,
                    MCW::MC_2 => 2,
                    MCW::MC_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _MCW<'a> {
            w: &'a mut W,
        }
        impl<'a> _MCW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: MCW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Timer A mode control: 0 - Stop"]
            #[inline(always)]
            pub fn mc_0(self) -> &'a mut W {
                self.variant(MCW::MC_0)
            }
            #[doc = "Timer A mode control: 1 - Up to CCR0"]
            #[inline(always)]
            pub fn mc_1(self) -> &'a mut W {
                self.variant(MCW::MC_1)
            }
            #[doc = "Timer A mode control: 2 - Continous up"]
            #[inline(always)]
            pub fn mc_2(self) -> &'a mut W {
                self.variant(MCW::MC_2)
            }
            #[doc = "Timer A mode control: 3 - Up/Down"]
            #[inline(always)]
            pub fn mc_3(self) -> &'a mut W {
                self.variant(MCW::MC_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `ID`"]
        pub enum IDW {
            #[doc = "Timer A input divider: 0 - /1"]
            ID_0,
            #[doc = "Timer A input divider: 1 - /2"]
            ID_1,
            #[doc = "Timer A input divider: 2 - /4"]
            ID_2,
            #[doc = "Timer A input divider: 3 - /8"]
            ID_3,
        }
        impl IDW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    IDW::ID_0 => 0,
                    IDW::ID_1 => 1,
                    IDW::ID_2 => 2,
                    IDW::ID_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _IDW<'a> {
            w: &'a mut W,
        }
        impl<'a> _IDW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: IDW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Timer A input divider: 0 - /1"]
            #[inline(always)]
            pub fn id_0(self) -> &'a mut W {
                self.variant(IDW::ID_0)
            }
            #[doc = "Timer A input divider: 1 - /2"]
            #[inline(always)]
            pub fn id_1(self) -> &'a mut W {
                self.variant(IDW::ID_1)
            }
            #[doc = "Timer A input divider: 2 - /4"]
            #[inline(always)]
            pub fn id_2(self) -> &'a mut W {
                self.variant(IDW::ID_2)
            }
            #[doc = "Timer A input divider: 3 - /8"]
            #[inline(always)]
            pub fn id_3(self) -> &'a mut W {
                self.variant(IDW::ID_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 6;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `TASSEL`"]
        pub enum TASSELW {
            #[doc = "Timer A clock source select: 0 - TACLK"]
            TASSEL_0,
            #[doc = "Timer A clock source select: 1 - ACLK"]
            TASSEL_1,
            #[doc = "Timer A clock source select: 2 - SMCLK"]
            TASSEL_2,
            #[doc = "Timer A clock source select: 3 - INCLK"]
            TASSEL_3,
        }
        impl TASSELW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    TASSELW::TASSEL_0 => 0,
                    TASSELW::TASSEL_1 => 1,
                    TASSELW::TASSEL_2 => 2,
                    TASSELW::TASSEL_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _TASSELW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TASSELW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: TASSELW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Timer A clock source select: 0 - TACLK"]
            #[inline(always)]
            pub fn tassel_0(self) -> &'a mut W {
                self.variant(TASSELW::TASSEL_0)
            }
            #[doc = "Timer A clock source select: 1 - ACLK"]
            #[inline(always)]
            pub fn tassel_1(self) -> &'a mut W {
                self.variant(TASSELW::TASSEL_1)
            }
            #[doc = "Timer A clock source select: 2 - SMCLK"]
            #[inline(always)]
            pub fn tassel_2(self) -> &'a mut W {
                self.variant(TASSELW::TASSEL_2)
            }
            #[doc = "Timer A clock source select: 3 - INCLK"]
            #[inline(always)]
            pub fn tassel_3(self) -> &'a mut W {
                self.variant(TASSELW::TASSEL_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 8;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
            #[doc = "Bit 0 - Timer A counter interrupt flag"]
            #[inline(always)]
            pub fn taifg(&self) -> TAIFGR {
                TAIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 1 - Timer A counter interrupt enable"]
            #[inline(always)]
            pub fn taie(&self) -> TAIER {
                TAIER::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 2 - Timer A counter clear"]
            #[inline(always)]
            pub fn taclr(&self) -> TACLRR {
                TACLRR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bits 4:5 - Timer A mode control 1"]
            #[inline(always)]
            pub fn mc(&self) -> MCR {
                MCR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
            #[doc = "Bits 6:7 - Timer A clock input divider 1"]
            #[inline(always)]
            pub fn id(&self) -> IDR {
                IDR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 6;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
            #[doc = "Bits 8:9 - Timer A clock source select 1"]
            #[inline(always)]
            pub fn tassel(&self) -> TASSELR {
                TASSELR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 8;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Timer A counter interrupt flag"]
            #[inline(always)]
            pub fn taifg(&mut self) -> _TAIFGW {
                _TAIFGW { w: self }
            }
            #[doc = "Bit 1 - Timer A counter interrupt enable"]
            #[inline(always)]
            pub fn taie(&mut self) -> _TAIEW {
                _TAIEW { w: self }
            }
            #[doc = "Bit 2 - Timer A counter clear"]
            #[inline(always)]
            pub fn taclr(&mut self) -> _TACLRW {
                _TACLRW { w: self }
            }
            #[doc = "Bits 4:5 - Timer A mode control 1"]
            #[inline(always)]
            pub fn mc(&mut self) -> _MCW {
                _MCW { w: self }
            }
            #[doc = "Bits 6:7 - Timer A clock input divider 1"]
            #[inline(always)]
            pub fn id(&mut self) -> _IDW {
                _IDW { w: self }
            }
            #[doc = "Bits 8:9 - Timer A clock source select 1"]
            #[inline(always)]
            pub fn tassel(&mut self) -> _TASSELW {
                _TASSELW { w: self }
            }
        }
    }
    #[doc = "Timer A Capture/Compare Control 0"]
    pub struct TACCTL0 {
        register: VolatileCell<u16>,
    }
    #[doc = "Timer A Capture/Compare Control 0"]
    pub mod tacctl0 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::TACCTL0 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `CCIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CCIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CCIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CCIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CCIFGR {
                match value {
                    i => CCIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `COV`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum COVR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl COVR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    COVR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> COVR {
                match value {
                    i => COVR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `OUT`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum OUTR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl OUTR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    OUTR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> OUTR {
                match value {
                    i => OUTR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CCI`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CCIR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CCIR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CCIR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CCIR {
                match value {
                    i => CCIR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CCIE`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CCIER {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CCIER {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CCIER::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CCIER {
                match value {
                    i => CCIER::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `OUTMOD`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum OUTMODR {
            #[doc = "PWM output mode: 0 - output only"]
            OUTMOD_0,
            #[doc = "PWM output mode: 1 - set"]
            OUTMOD_1,
            #[doc = "PWM output mode: 2 - PWM toggle/reset"]
            OUTMOD_2,
            #[doc = "PWM output mode: 3 - PWM set/reset"]
            OUTMOD_3,
            #[doc = "PWM output mode: 4 - toggle"]
            OUTMOD_4,
            #[doc = "PWM output mode: 5 - Reset"]
            OUTMOD_5,
            #[doc = "PWM output mode: 6 - PWM toggle/set"]
            OUTMOD_6,
            #[doc = "PWM output mode: 7 - PWM reset/set"]
            OUTMOD_7,
        }
        impl OUTMODR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    OUTMODR::OUTMOD_0 => 0,
                    OUTMODR::OUTMOD_1 => 1,
                    OUTMODR::OUTMOD_2 => 2,
                    OUTMODR::OUTMOD_3 => 3,
                    OUTMODR::OUTMOD_4 => 4,
                    OUTMODR::OUTMOD_5 => 5,
                    OUTMODR::OUTMOD_6 => 6,
                    OUTMODR::OUTMOD_7 => 7,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> OUTMODR {
                match value {
                    0 => OUTMODR::OUTMOD_0,
                    1 => OUTMODR::OUTMOD_1,
                    2 => OUTMODR::OUTMOD_2,
                    3 => OUTMODR::OUTMOD_3,
                    4 => OUTMODR::OUTMOD_4,
                    5 => OUTMODR::OUTMOD_5,
                    6 => OUTMODR::OUTMOD_6,
                    7 => OUTMODR::OUTMOD_7,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `OUTMOD_0`"]
            #[inline(always)]
            pub fn is_outmod_0(&self) -> bool {
                *self == OUTMODR::OUTMOD_0
            }
            #[doc = "Checks if the value of the field is `OUTMOD_1`"]
            #[inline(always)]
            pub fn is_outmod_1(&self) -> bool {
                *self == OUTMODR::OUTMOD_1
            }
            #[doc = "Checks if the value of the field is `OUTMOD_2`"]
            #[inline(always)]
            pub fn is_outmod_2(&self) -> bool {
                *self == OUTMODR::OUTMOD_2
            }
            #[doc = "Checks if the value of the field is `OUTMOD_3`"]
            #[inline(always)]
            pub fn is_outmod_3(&self) -> bool {
                *self == OUTMODR::OUTMOD_3
            }
            #[doc = "Checks if the value of the field is `OUTMOD_4`"]
            #[inline(always)]
            pub fn is_outmod_4(&self) -> bool {
                *self == OUTMODR::OUTMOD_4
            }
            #[doc = "Checks if the value of the field is `OUTMOD_5`"]
            #[inline(always)]
            pub fn is_outmod_5(&self) -> bool {
                *self == OUTMODR::OUTMOD_5
            }
            #[doc = "Checks if the value of the field is `OUTMOD_6`"]
            #[inline(always)]
            pub fn is_outmod_6(&self) -> bool {
                *self == OUTMODR::OUTMOD_6
            }
            #[doc = "Checks if the value of the field is `OUTMOD_7`"]
            #[inline(always)]
            pub fn is_outmod_7(&self) -> bool {
                *self == OUTMODR::OUTMOD_7
            }
        }
        #[doc = "Possible values of the field `CAP`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPR {
                match value {
                    i => CAPR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `SCCI`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum SCCIR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl SCCIR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    SCCIR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> SCCIR {
                match value {
                    i => SCCIR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `SCS`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum SCSR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl SCSR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    SCSR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> SCSR {
                match value {
                    i => SCSR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CCIS`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CCISR {
            #[doc = "Capture input select: 0 - CCIxA"]
            CCIS_0,
            #[doc = "Capture input select: 1 - CCIxB"]
            CCIS_1,
            #[doc = "Capture input select: 2 - GND"]
            CCIS_2,
            #[doc = "Capture input select: 3 - Vcc"]
            CCIS_3,
        }
        impl CCISR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    CCISR::CCIS_0 => 0,
                    CCISR::CCIS_1 => 1,
                    CCISR::CCIS_2 => 2,
                    CCISR::CCIS_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> CCISR {
                match value {
                    0 => CCISR::CCIS_0,
                    1 => CCISR::CCIS_1,
                    2 => CCISR::CCIS_2,
                    3 => CCISR::CCIS_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `CCIS_0`"]
            #[inline(always)]
            pub fn is_ccis_0(&self) -> bool {
                *self == CCISR::CCIS_0
            }
            #[doc = "Checks if the value of the field is `CCIS_1`"]
            #[inline(always)]
            pub fn is_ccis_1(&self) -> bool {
                *self == CCISR::CCIS_1
            }
            #[doc = "Checks if the value of the field is `CCIS_2`"]
            #[inline(always)]
            pub fn is_ccis_2(&self) -> bool {
                *self == CCISR::CCIS_2
            }
            #[doc = "Checks if the value of the field is `CCIS_3`"]
            #[inline(always)]
            pub fn is_ccis_3(&self) -> bool {
                *self == CCISR::CCIS_3
            }
        }
        #[doc = "Possible values of the field `CM`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CMR {
            #[doc = "Capture mode: 0 - disabled"]
            CM_0,
            #[doc = "Capture mode: 1 - pos. edge"]
            CM_1,
            #[doc = "Capture mode: 1 - neg. edge"]
            CM_2,
            #[doc = "Capture mode: 1 - both edges"]
            CM_3,
        }
        impl CMR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    CMR::CM_0 => 0,
                    CMR::CM_1 => 1,
                    CMR::CM_2 => 2,
                    CMR::CM_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> CMR {
                match value {
                    0 => CMR::CM_0,
                    1 => CMR::CM_1,
                    2 => CMR::CM_2,
                    3 => CMR::CM_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `CM_0`"]
            #[inline(always)]
            pub fn is_cm_0(&self) -> bool {
                *self == CMR::CM_0
            }
            #[doc = "Checks if the value of the field is `CM_1`"]
            #[inline(always)]
            pub fn is_cm_1(&self) -> bool {
                *self == CMR::CM_1
            }
            #[doc = "Checks if the value of the field is `CM_2`"]
            #[inline(always)]
            pub fn is_cm_2(&self) -> bool {
                *self == CMR::CM_2
            }
            #[doc = "Checks if the value of the field is `CM_3`"]
            #[inline(always)]
            pub fn is_cm_3(&self) -> bool {
                *self == CMR::CM_3
            }
        }
        #[doc = "Values that can be written to the field `CCIFG`"]
        pub enum CCIFGW { }
        impl CCIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CCIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CCIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CCIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `COV`"]
        pub enum COVW { }
        impl COVW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _COVW<'a> {
            w: &'a mut W,
        }
        impl<'a> _COVW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: COVW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `OUT`"]
        pub enum OUTW { }
        impl OUTW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _OUTW<'a> {
            w: &'a mut W,
        }
        impl<'a> _OUTW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: OUTW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CCI`"]
        pub enum CCIW { }
        impl CCIW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CCIW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CCIW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CCIW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CCIE`"]
        pub enum CCIEW { }
        impl CCIEW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CCIEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CCIEW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CCIEW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `OUTMOD`"]
        pub enum OUTMODW {
            #[doc = "PWM output mode: 0 - output only"]
            OUTMOD_0,
            #[doc = "PWM output mode: 1 - set"]
            OUTMOD_1,
            #[doc = "PWM output mode: 2 - PWM toggle/reset"]
            OUTMOD_2,
            #[doc = "PWM output mode: 3 - PWM set/reset"]
            OUTMOD_3,
            #[doc = "PWM output mode: 4 - toggle"]
            OUTMOD_4,
            #[doc = "PWM output mode: 5 - Reset"]
            OUTMOD_5,
            #[doc = "PWM output mode: 6 - PWM toggle/set"]
            OUTMOD_6,
            #[doc = "PWM output mode: 7 - PWM reset/set"]
            OUTMOD_7,
        }
        impl OUTMODW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    OUTMODW::OUTMOD_0 => 0,
                    OUTMODW::OUTMOD_1 => 1,
                    OUTMODW::OUTMOD_2 => 2,
                    OUTMODW::OUTMOD_3 => 3,
                    OUTMODW::OUTMOD_4 => 4,
                    OUTMODW::OUTMOD_5 => 5,
                    OUTMODW::OUTMOD_6 => 6,
                    OUTMODW::OUTMOD_7 => 7,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _OUTMODW<'a> {
            w: &'a mut W,
        }
        impl<'a> _OUTMODW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: OUTMODW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "PWM output mode: 0 - output only"]
            #[inline(always)]
            pub fn outmod_0(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_0)
            }
            #[doc = "PWM output mode: 1 - set"]
            #[inline(always)]
            pub fn outmod_1(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_1)
            }
            #[doc = "PWM output mode: 2 - PWM toggle/reset"]
            #[inline(always)]
            pub fn outmod_2(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_2)
            }
            #[doc = "PWM output mode: 3 - PWM set/reset"]
            #[inline(always)]
            pub fn outmod_3(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_3)
            }
            #[doc = "PWM output mode: 4 - toggle"]
            #[inline(always)]
            pub fn outmod_4(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_4)
            }
            #[doc = "PWM output mode: 5 - Reset"]
            #[inline(always)]
            pub fn outmod_5(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_5)
            }
            #[doc = "PWM output mode: 6 - PWM toggle/set"]
            #[inline(always)]
            pub fn outmod_6(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_6)
            }
            #[doc = "PWM output mode: 7 - PWM reset/set"]
            #[inline(always)]
            pub fn outmod_7(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_7)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 7;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAP`"]
        pub enum CAPW { }
        impl CAPW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `SCCI`"]
        pub enum SCCIW { }
        impl SCCIW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _SCCIW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SCCIW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: SCCIW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `SCS`"]
        pub enum SCSW { }
        impl SCSW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _SCSW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SCSW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: SCSW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CCIS`"]
        pub enum CCISW {
            #[doc = "Capture input select: 0 - CCIxA"]
            CCIS_0,
            #[doc = "Capture input select: 1 - CCIxB"]
            CCIS_1,
            #[doc = "Capture input select: 2 - GND"]
            CCIS_2,
            #[doc = "Capture input select: 3 - Vcc"]
            CCIS_3,
        }
        impl CCISW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    CCISW::CCIS_0 => 0,
                    CCISW::CCIS_1 => 1,
                    CCISW::CCIS_2 => 2,
                    CCISW::CCIS_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CCISW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CCISW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CCISW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Capture input select: 0 - CCIxA"]
            #[inline(always)]
            pub fn ccis_0(self) -> &'a mut W {
                self.variant(CCISW::CCIS_0)
            }
            #[doc = "Capture input select: 1 - CCIxB"]
            #[inline(always)]
            pub fn ccis_1(self) -> &'a mut W {
                self.variant(CCISW::CCIS_1)
            }
            #[doc = "Capture input select: 2 - GND"]
            #[inline(always)]
            pub fn ccis_2(self) -> &'a mut W {
                self.variant(CCISW::CCIS_2)
            }
            #[doc = "Capture input select: 3 - Vcc"]
            #[inline(always)]
            pub fn ccis_3(self) -> &'a mut W {
                self.variant(CCISW::CCIS_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CM`"]
        pub enum CMW {
            #[doc = "Capture mode: 0 - disabled"]
            CM_0,
            #[doc = "Capture mode: 1 - pos. edge"]
            CM_1,
            #[doc = "Capture mode: 1 - neg. edge"]
            CM_2,
            #[doc = "Capture mode: 1 - both edges"]
            CM_3,
        }
        impl CMW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    CMW::CM_0 => 0,
                    CMW::CM_1 => 1,
                    CMW::CM_2 => 2,
                    CMW::CM_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CMW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CMW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CMW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Capture mode: 0 - disabled"]
            #[inline(always)]
            pub fn cm_0(self) -> &'a mut W {
                self.variant(CMW::CM_0)
            }
            #[doc = "Capture mode: 1 - pos. edge"]
            #[inline(always)]
            pub fn cm_1(self) -> &'a mut W {
                self.variant(CMW::CM_1)
            }
            #[doc = "Capture mode: 1 - neg. edge"]
            #[inline(always)]
            pub fn cm_2(self) -> &'a mut W {
                self.variant(CMW::CM_2)
            }
            #[doc = "Capture mode: 1 - both edges"]
            #[inline(always)]
            pub fn cm_3(self) -> &'a mut W {
                self.variant(CMW::CM_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 14;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
            #[doc = "Bit 0 - Capture/compare interrupt flag"]
            #[inline(always)]
            pub fn ccifg(&self) -> CCIFGR {
                CCIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 1 - Capture/compare overflow flag"]
            #[inline(always)]
            pub fn cov(&self) -> COVR {
                COVR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 2 - PWM Output signal if output mode 0"]
            #[inline(always)]
            pub fn out(&self) -> OUTR {
                OUTR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 3 - Capture input signal (read)"]
            #[inline(always)]
            pub fn cci(&self) -> CCIR {
                CCIR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 4 - Capture/compare interrupt enable"]
            #[inline(always)]
            pub fn ccie(&self) -> CCIER {
                CCIER::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bits 5:7 - Output mode 2"]
            #[inline(always)]
            pub fn outmod(&self) -> OUTMODR {
                OUTMODR::_from({
                    const MASK: u8 = 7;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
            #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
            #[inline(always)]
            pub fn cap(&self) -> CAPR {
                CAPR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 8;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 10 - Latched capture signal (read)"]
            #[inline(always)]
            pub fn scci(&self) -> SCCIR {
                SCCIR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 10;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 11 - Capture sychronize"]
            #[inline(always)]
            pub fn scs(&self) -> SCSR {
                SCSR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 11;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bits 12:13 - Capture input select 1"]
            #[inline(always)]
            pub fn ccis(&self) -> CCISR {
                CCISR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 12;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
            #[doc = "Bits 14:15 - Capture mode 1"]
            #[inline(always)]
            pub fn cm(&self) -> CMR {
                CMR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 14;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Capture/compare interrupt flag"]
            #[inline(always)]
            pub fn ccifg(&mut self) -> _CCIFGW {
                _CCIFGW { w: self }
            }
            #[doc = "Bit 1 - Capture/compare overflow flag"]
            #[inline(always)]
            pub fn cov(&mut self) -> _COVW {
                _COVW { w: self }
            }
            #[doc = "Bit 2 - PWM Output signal if output mode 0"]
            #[inline(always)]
            pub fn out(&mut self) -> _OUTW {
                _OUTW { w: self }
            }
            #[doc = "Bit 3 - Capture input signal (read)"]
            #[inline(always)]
            pub fn cci(&mut self) -> _CCIW {
                _CCIW { w: self }
            }
            #[doc = "Bit 4 - Capture/compare interrupt enable"]
            #[inline(always)]
            pub fn ccie(&mut self) -> _CCIEW {
                _CCIEW { w: self }
            }
            #[doc = "Bits 5:7 - Output mode 2"]
            #[inline(always)]
            pub fn outmod(&mut self) -> _OUTMODW {
                _OUTMODW { w: self }
            }
            #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
            #[inline(always)]
            pub fn cap(&mut self) -> _CAPW {
                _CAPW { w: self }
            }
            #[doc = "Bit 10 - Latched capture signal (read)"]
            #[inline(always)]
            pub fn scci(&mut self) -> _SCCIW {
                _SCCIW { w: self }
            }
            #[doc = "Bit 11 - Capture sychronize"]
            #[inline(always)]
            pub fn scs(&mut self) -> _SCSW {
                _SCSW { w: self }
            }
            #[doc = "Bits 12:13 - Capture input select 1"]
            #[inline(always)]
            pub fn ccis(&mut self) -> _CCISW {
                _CCISW { w: self }
            }
            #[doc = "Bits 14:15 - Capture mode 1"]
            #[inline(always)]
            pub fn cm(&mut self) -> _CMW {
                _CMW { w: self }
            }
        }
    }
    #[doc = "Timer A Capture/Compare Control 1"]
    pub struct TACCTL1 {
        register: VolatileCell<u16>,
    }
    #[doc = "Timer A Capture/Compare Control 1"]
    pub mod tacctl1 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::TACCTL1 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `CCIFG`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CCIFGR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CCIFGR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CCIFGR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CCIFGR {
                match value {
                    i => CCIFGR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `COV`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum COVR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl COVR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    COVR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> COVR {
                match value {
                    i => COVR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `OUT`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum OUTR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl OUTR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    OUTR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> OUTR {
                match value {
                    i => OUTR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CCI`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CCIR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CCIR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CCIR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CCIR {
                match value {
                    i => CCIR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CCIE`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CCIER {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CCIER {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CCIER::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CCIER {
                match value {
                    i => CCIER::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `OUTMOD`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum OUTMODR {
            #[doc = "PWM output mode: 0 - output only"]
            OUTMOD_0,
            #[doc = "PWM output mode: 1 - set"]
            OUTMOD_1,
            #[doc = "PWM output mode: 2 - PWM toggle/reset"]
            OUTMOD_2,
            #[doc = "PWM output mode: 3 - PWM set/reset"]
            OUTMOD_3,
            #[doc = "PWM output mode: 4 - toggle"]
            OUTMOD_4,
            #[doc = "PWM output mode: 5 - Reset"]
            OUTMOD_5,
            #[doc = "PWM output mode: 6 - PWM toggle/set"]
            OUTMOD_6,
            #[doc = "PWM output mode: 7 - PWM reset/set"]
            OUTMOD_7,
        }
        impl OUTMODR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    OUTMODR::OUTMOD_0 => 0,
                    OUTMODR::OUTMOD_1 => 1,
                    OUTMODR::OUTMOD_2 => 2,
                    OUTMODR::OUTMOD_3 => 3,
                    OUTMODR::OUTMOD_4 => 4,
                    OUTMODR::OUTMOD_5 => 5,
                    OUTMODR::OUTMOD_6 => 6,
                    OUTMODR::OUTMOD_7 => 7,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> OUTMODR {
                match value {
                    0 => OUTMODR::OUTMOD_0,
                    1 => OUTMODR::OUTMOD_1,
                    2 => OUTMODR::OUTMOD_2,
                    3 => OUTMODR::OUTMOD_3,
                    4 => OUTMODR::OUTMOD_4,
                    5 => OUTMODR::OUTMOD_5,
                    6 => OUTMODR::OUTMOD_6,
                    7 => OUTMODR::OUTMOD_7,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `OUTMOD_0`"]
            #[inline(always)]
            pub fn is_outmod_0(&self) -> bool {
                *self == OUTMODR::OUTMOD_0
            }
            #[doc = "Checks if the value of the field is `OUTMOD_1`"]
            #[inline(always)]
            pub fn is_outmod_1(&self) -> bool {
                *self == OUTMODR::OUTMOD_1
            }
            #[doc = "Checks if the value of the field is `OUTMOD_2`"]
            #[inline(always)]
            pub fn is_outmod_2(&self) -> bool {
                *self == OUTMODR::OUTMOD_2
            }
            #[doc = "Checks if the value of the field is `OUTMOD_3`"]
            #[inline(always)]
            pub fn is_outmod_3(&self) -> bool {
                *self == OUTMODR::OUTMOD_3
            }
            #[doc = "Checks if the value of the field is `OUTMOD_4`"]
            #[inline(always)]
            pub fn is_outmod_4(&self) -> bool {
                *self == OUTMODR::OUTMOD_4
            }
            #[doc = "Checks if the value of the field is `OUTMOD_5`"]
            #[inline(always)]
            pub fn is_outmod_5(&self) -> bool {
                *self == OUTMODR::OUTMOD_5
            }
            #[doc = "Checks if the value of the field is `OUTMOD_6`"]
            #[inline(always)]
            pub fn is_outmod_6(&self) -> bool {
                *self == OUTMODR::OUTMOD_6
            }
            #[doc = "Checks if the value of the field is `OUTMOD_7`"]
            #[inline(always)]
            pub fn is_outmod_7(&self) -> bool {
                *self == OUTMODR::OUTMOD_7
            }
        }
        #[doc = "Possible values of the field `CAP`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CAPR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl CAPR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    CAPR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> CAPR {
                match value {
                    i => CAPR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `SCCI`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum SCCIR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl SCCIR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    SCCIR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> SCCIR {
                match value {
                    i => SCCIR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `SCS`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum SCSR {
            #[doc = r" Reserved"]
            _Reserved(bool),
        }
        impl SCSR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    SCSR::_Reserved(bits) => bits,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> SCSR {
                match value {
                    i => SCSR::_Reserved(i),
                }
            }
        }
        #[doc = "Possible values of the field `CCIS`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CCISR {
            #[doc = "Capture input select: 0 - CCIxA"]
            CCIS_0,
            #[doc = "Capture input select: 1 - CCIxB"]
            CCIS_1,
            #[doc = "Capture input select: 2 - GND"]
            CCIS_2,
            #[doc = "Capture input select: 3 - Vcc"]
            CCIS_3,
        }
        impl CCISR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    CCISR::CCIS_0 => 0,
                    CCISR::CCIS_1 => 1,
                    CCISR::CCIS_2 => 2,
                    CCISR::CCIS_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> CCISR {
                match value {
                    0 => CCISR::CCIS_0,
                    1 => CCISR::CCIS_1,
                    2 => CCISR::CCIS_2,
                    3 => CCISR::CCIS_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `CCIS_0`"]
            #[inline(always)]
            pub fn is_ccis_0(&self) -> bool {
                *self == CCISR::CCIS_0
            }
            #[doc = "Checks if the value of the field is `CCIS_1`"]
            #[inline(always)]
            pub fn is_ccis_1(&self) -> bool {
                *self == CCISR::CCIS_1
            }
            #[doc = "Checks if the value of the field is `CCIS_2`"]
            #[inline(always)]
            pub fn is_ccis_2(&self) -> bool {
                *self == CCISR::CCIS_2
            }
            #[doc = "Checks if the value of the field is `CCIS_3`"]
            #[inline(always)]
            pub fn is_ccis_3(&self) -> bool {
                *self == CCISR::CCIS_3
            }
        }
        #[doc = "Possible values of the field `CM`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CMR {
            #[doc = "Capture mode: 0 - disabled"]
            CM_0,
            #[doc = "Capture mode: 1 - pos. edge"]
            CM_1,
            #[doc = "Capture mode: 1 - neg. edge"]
            CM_2,
            #[doc = "Capture mode: 1 - both edges"]
            CM_3,
        }
        impl CMR {
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                match *self {
                    CMR::CM_0 => 0,
                    CMR::CM_1 => 1,
                    CMR::CM_2 => 2,
                    CMR::CM_3 => 3,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: u8) -> CMR {
                match value {
                    0 => CMR::CM_0,
                    1 => CMR::CM_1,
                    2 => CMR::CM_2,
                    3 => CMR::CM_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Checks if the value of the field is `CM_0`"]
            #[inline(always)]
            pub fn is_cm_0(&self) -> bool {
                *self == CMR::CM_0
            }
            #[doc = "Checks if the value of the field is `CM_1`"]
            #[inline(always)]
            pub fn is_cm_1(&self) -> bool {
                *self == CMR::CM_1
            }
            #[doc = "Checks if the value of the field is `CM_2`"]
            #[inline(always)]
            pub fn is_cm_2(&self) -> bool {
                *self == CMR::CM_2
            }
            #[doc = "Checks if the value of the field is `CM_3`"]
            #[inline(always)]
            pub fn is_cm_3(&self) -> bool {
                *self == CMR::CM_3
            }
        }
        #[doc = "Values that can be written to the field `CCIFG`"]
        pub enum CCIFGW { }
        impl CCIFGW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CCIFGW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CCIFGW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CCIFGW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `COV`"]
        pub enum COVW { }
        impl COVW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _COVW<'a> {
            w: &'a mut W,
        }
        impl<'a> _COVW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: COVW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `OUT`"]
        pub enum OUTW { }
        impl OUTW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _OUTW<'a> {
            w: &'a mut W,
        }
        impl<'a> _OUTW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: OUTW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 2;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CCI`"]
        pub enum CCIW { }
        impl CCIW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CCIW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CCIW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CCIW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 3;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CCIE`"]
        pub enum CCIEW { }
        impl CCIEW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CCIEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CCIEW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CCIEW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 4;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `OUTMOD`"]
        pub enum OUTMODW {
            #[doc = "PWM output mode: 0 - output only"]
            OUTMOD_0,
            #[doc = "PWM output mode: 1 - set"]
            OUTMOD_1,
            #[doc = "PWM output mode: 2 - PWM toggle/reset"]
            OUTMOD_2,
            #[doc = "PWM output mode: 3 - PWM set/reset"]
            OUTMOD_3,
            #[doc = "PWM output mode: 4 - toggle"]
            OUTMOD_4,
            #[doc = "PWM output mode: 5 - Reset"]
            OUTMOD_5,
            #[doc = "PWM output mode: 6 - PWM toggle/set"]
            OUTMOD_6,
            #[doc = "PWM output mode: 7 - PWM reset/set"]
            OUTMOD_7,
        }
        impl OUTMODW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    OUTMODW::OUTMOD_0 => 0,
                    OUTMODW::OUTMOD_1 => 1,
                    OUTMODW::OUTMOD_2 => 2,
                    OUTMODW::OUTMOD_3 => 3,
                    OUTMODW::OUTMOD_4 => 4,
                    OUTMODW::OUTMOD_5 => 5,
                    OUTMODW::OUTMOD_6 => 6,
                    OUTMODW::OUTMOD_7 => 7,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _OUTMODW<'a> {
            w: &'a mut W,
        }
        impl<'a> _OUTMODW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: OUTMODW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "PWM output mode: 0 - output only"]
            #[inline(always)]
            pub fn outmod_0(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_0)
            }
            #[doc = "PWM output mode: 1 - set"]
            #[inline(always)]
            pub fn outmod_1(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_1)
            }
            #[doc = "PWM output mode: 2 - PWM toggle/reset"]
            #[inline(always)]
            pub fn outmod_2(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_2)
            }
            #[doc = "PWM output mode: 3 - PWM set/reset"]
            #[inline(always)]
            pub fn outmod_3(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_3)
            }
            #[doc = "PWM output mode: 4 - toggle"]
            #[inline(always)]
            pub fn outmod_4(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_4)
            }
            #[doc = "PWM output mode: 5 - Reset"]
            #[inline(always)]
            pub fn outmod_5(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_5)
            }
            #[doc = "PWM output mode: 6 - PWM toggle/set"]
            #[inline(always)]
            pub fn outmod_6(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_6)
            }
            #[doc = "PWM output mode: 7 - PWM reset/set"]
            #[inline(always)]
            pub fn outmod_7(self) -> &'a mut W {
                self.variant(OUTMODW::OUTMOD_7)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 7;
                const OFFSET: u8 = 5;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CAP`"]
        pub enum CAPW { }
        impl CAPW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CAPW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CAPW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CAPW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 8;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `SCCI`"]
        pub enum SCCIW { }
        impl SCCIW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _SCCIW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SCCIW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: SCCIW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 10;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `SCS`"]
        pub enum SCSW { }
        impl SCSW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match * self { }
            }
        }
        #[doc = r" Proxy"]
        pub struct _SCSW<'a> {
            w: &'a mut W,
        }
        impl<'a> _SCSW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: SCSW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 11;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CCIS`"]
        pub enum CCISW {
            #[doc = "Capture input select: 0 - CCIxA"]
            CCIS_0,
            #[doc = "Capture input select: 1 - CCIxB"]
            CCIS_1,
            #[doc = "Capture input select: 2 - GND"]
            CCIS_2,
            #[doc = "Capture input select: 3 - Vcc"]
            CCIS_3,
        }
        impl CCISW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    CCISW::CCIS_0 => 0,
                    CCISW::CCIS_1 => 1,
                    CCISW::CCIS_2 => 2,
                    CCISW::CCIS_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CCISW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CCISW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CCISW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Capture input select: 0 - CCIxA"]
            #[inline(always)]
            pub fn ccis_0(self) -> &'a mut W {
                self.variant(CCISW::CCIS_0)
            }
            #[doc = "Capture input select: 1 - CCIxB"]
            #[inline(always)]
            pub fn ccis_1(self) -> &'a mut W {
                self.variant(CCISW::CCIS_1)
            }
            #[doc = "Capture input select: 2 - GND"]
            #[inline(always)]
            pub fn ccis_2(self) -> &'a mut W {
                self.variant(CCISW::CCIS_2)
            }
            #[doc = "Capture input select: 3 - Vcc"]
            #[inline(always)]
            pub fn ccis_3(self) -> &'a mut W {
                self.variant(CCISW::CCIS_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 12;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `CM`"]
        pub enum CMW {
            #[doc = "Capture mode: 0 - disabled"]
            CM_0,
            #[doc = "Capture mode: 1 - pos. edge"]
            CM_1,
            #[doc = "Capture mode: 1 - neg. edge"]
            CM_2,
            #[doc = "Capture mode: 1 - both edges"]
            CM_3,
        }
        impl CMW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> u8 {
                match *self {
                    CMW::CM_0 => 0,
                    CMW::CM_1 => 1,
                    CMW::CM_2 => 2,
                    CMW::CM_3 => 3,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _CMW<'a> {
            w: &'a mut W,
        }
        impl<'a> _CMW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: CMW) -> &'a mut W {
                {
                    self.bits(variant._bits())
                }
            }
            #[doc = "Capture mode: 0 - disabled"]
            #[inline(always)]
            pub fn cm_0(self) -> &'a mut W {
                self.variant(CMW::CM_0)
            }
            #[doc = "Capture mode: 1 - pos. edge"]
            #[inline(always)]
            pub fn cm_1(self) -> &'a mut W {
                self.variant(CMW::CM_1)
            }
            #[doc = "Capture mode: 1 - neg. edge"]
            #[inline(always)]
            pub fn cm_2(self) -> &'a mut W {
                self.variant(CMW::CM_2)
            }
            #[doc = "Capture mode: 1 - both edges"]
            #[inline(always)]
            pub fn cm_3(self) -> &'a mut W {
                self.variant(CMW::CM_3)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bits(self, value: u8) -> &'a mut W {
                const MASK: u8 = 3;
                const OFFSET: u8 = 14;
                self.w.bits &= !((MASK as u16) << OFFSET);
                self.w.bits |= ((value & MASK) as u16) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
            #[doc = "Bit 0 - Capture/compare interrupt flag"]
            #[inline(always)]
            pub fn ccifg(&self) -> CCIFGR {
                CCIFGR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 1 - Capture/compare overflow flag"]
            #[inline(always)]
            pub fn cov(&self) -> COVR {
                COVR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 1;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 2 - PWM Output signal if output mode 0"]
            #[inline(always)]
            pub fn out(&self) -> OUTR {
                OUTR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 2;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 3 - Capture input signal (read)"]
            #[inline(always)]
            pub fn cci(&self) -> CCIR {
                CCIR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 3;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 4 - Capture/compare interrupt enable"]
            #[inline(always)]
            pub fn ccie(&self) -> CCIER {
                CCIER::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 4;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bits 5:7 - Output mode 2"]
            #[inline(always)]
            pub fn outmod(&self) -> OUTMODR {
                OUTMODR::_from({
                    const MASK: u8 = 7;
                    const OFFSET: u8 = 5;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
            #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
            #[inline(always)]
            pub fn cap(&self) -> CAPR {
                CAPR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 8;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 10 - Latched capture signal (read)"]
            #[inline(always)]
            pub fn scci(&self) -> SCCIR {
                SCCIR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 10;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bit 11 - Capture sychronize"]
            #[inline(always)]
            pub fn scs(&self) -> SCSR {
                SCSR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 11;
                    ((self.bits >> OFFSET) & MASK as u16) != 0
                })
            }
            #[doc = "Bits 12:13 - Capture input select 1"]
            #[inline(always)]
            pub fn ccis(&self) -> CCISR {
                CCISR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 12;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
            #[doc = "Bits 14:15 - Capture mode 1"]
            #[inline(always)]
            pub fn cm(&self) -> CMR {
                CMR::_from({
                    const MASK: u8 = 3;
                    const OFFSET: u8 = 14;
                    ((self.bits >> OFFSET) & MASK as u16) as u8
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Capture/compare interrupt flag"]
            #[inline(always)]
            pub fn ccifg(&mut self) -> _CCIFGW {
                _CCIFGW { w: self }
            }
            #[doc = "Bit 1 - Capture/compare overflow flag"]
            #[inline(always)]
            pub fn cov(&mut self) -> _COVW {
                _COVW { w: self }
            }
            #[doc = "Bit 2 - PWM Output signal if output mode 0"]
            #[inline(always)]
            pub fn out(&mut self) -> _OUTW {
                _OUTW { w: self }
            }
            #[doc = "Bit 3 - Capture input signal (read)"]
            #[inline(always)]
            pub fn cci(&mut self) -> _CCIW {
                _CCIW { w: self }
            }
            #[doc = "Bit 4 - Capture/compare interrupt enable"]
            #[inline(always)]
            pub fn ccie(&mut self) -> _CCIEW {
                _CCIEW { w: self }
            }
            #[doc = "Bits 5:7 - Output mode 2"]
            #[inline(always)]
            pub fn outmod(&mut self) -> _OUTMODW {
                _OUTMODW { w: self }
            }
            #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
            #[inline(always)]
            pub fn cap(&mut self) -> _CAPW {
                _CAPW { w: self }
            }
            #[doc = "Bit 10 - Latched capture signal (read)"]
            #[inline(always)]
            pub fn scci(&mut self) -> _SCCIW {
                _SCCIW { w: self }
            }
            #[doc = "Bit 11 - Capture sychronize"]
            #[inline(always)]
            pub fn scs(&mut self) -> _SCSW {
                _SCSW { w: self }
            }
            #[doc = "Bits 12:13 - Capture input select 1"]
            #[inline(always)]
            pub fn ccis(&mut self) -> _CCISW {
                _CCISW { w: self }
            }
            #[doc = "Bits 14:15 - Capture mode 1"]
            #[inline(always)]
            pub fn cm(&mut self) -> _CMW {
                _CMW { w: self }
            }
        }
    }
    #[doc = "Timer A Counter Register"]
    pub struct TAR {
        register: VolatileCell<u16>,
    }
    #[doc = "Timer A Counter Register"]
    pub mod tar {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::TAR {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "Timer A Capture/Compare 0"]
    pub struct TACCR0 {
        register: VolatileCell<u16>,
    }
    #[doc = "Timer A Capture/Compare 0"]
    pub mod taccr0 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::TACCR0 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "Timer A Capture/Compare 1"]
    pub struct TACCR1 {
        register: VolatileCell<u16>,
    }
    #[doc = "Timer A Capture/Compare 1"]
    pub mod taccr1 {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u16,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u16,
        }
        impl super::TACCR1 {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u16 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
}
#[doc = "Timer A2"]
pub struct TIMER_A2 {
    register_block: timer_a2::RegisterBlock,
}
impl Deref for TIMER_A2 {
    type Target = timer_a2::RegisterBlock;
    fn deref(&self) -> &timer_a2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Calibration Data"]
pub const CALIBRATION_DATA: Peripheral<CALIBRATION_DATA> = unsafe { Peripheral::new(4350) };
#[doc = "Calibration Data"]
pub mod calibration_data {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - DCOCTL Calibration Data for 1MHz"]
        pub caldco_1mhz: CALDCO_1MHZ,
        #[doc = "0x01 - BCSCTL1 Calibration Data for 1MHz"]
        pub calbc1_1mhz: CALBC1_1MHZ,
    }
    #[doc = "DCOCTL Calibration Data for 1MHz"]
    pub struct CALDCO_1MHZ {
        register: VolatileCell<u8>,
    }
    #[doc = "DCOCTL Calibration Data for 1MHz"]
    pub mod caldco_1mhz {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::CALDCO_1MHZ {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
    #[doc = "BCSCTL1 Calibration Data for 1MHz"]
    pub struct CALBC1_1MHZ {
        register: VolatileCell<u8>,
    }
    #[doc = "BCSCTL1 Calibration Data for 1MHz"]
    pub mod calbc1_1mhz {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u8,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u8,
        }
        impl super::CALBC1_1MHZ {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R { bits: self.register.get() }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u8 {
                self.bits
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
                self.bits = bits;
                self
            }
        }
    }
}
#[doc = "Calibration Data"]
pub struct CALIBRATION_DATA {
    register_block: calibration_data::RegisterBlock,
}
impl Deref for CALIBRATION_DATA {
    type Target = calibration_data::RegisterBlock;
    fn deref(&self) -> &calibration_data::RegisterBlock {
        &self.register_block
    }
}
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals<'a> {
    #[doc = "SPECIAL_FUNCTION"]
    pub SPECIAL_FUNCTION: &'a SPECIAL_FUNCTION,
    #[doc = "COMPARATOR_A"]
    pub COMPARATOR_A: &'a COMPARATOR_A,
    #[doc = "PORT_1_2"]
    pub PORT_1_2: &'a PORT_1_2,
    #[doc = "FLASH"]
    pub FLASH: &'a FLASH,
    #[doc = "WATCHDOG_TIMER"]
    pub WATCHDOG_TIMER: &'a WATCHDOG_TIMER,
    #[doc = "SYSTEM_CLOCK"]
    pub SYSTEM_CLOCK: &'a SYSTEM_CLOCK,
    #[doc = "TIMER_A2"]
    pub TIMER_A2: &'a TIMER_A2,
    #[doc = "CALIBRATION_DATA"]
    pub CALIBRATION_DATA: &'a CALIBRATION_DATA,
}
impl<'a> Peripherals<'a> {
    #[doc = r" Grants access to all the peripherals"]
    pub unsafe fn all() -> Self {
        Peripherals {
            SPECIAL_FUNCTION: &*SPECIAL_FUNCTION.get(),
            COMPARATOR_A: &*COMPARATOR_A.get(),
            PORT_1_2: &*PORT_1_2.get(),
            FLASH: &*FLASH.get(),
            WATCHDOG_TIMER: &*WATCHDOG_TIMER.get(),
            SYSTEM_CLOCK: &*SYSTEM_CLOCK.get(),
            TIMER_A2: &*TIMER_A2.get(),
            CALIBRATION_DATA: &*CALIBRATION_DATA.get(),
        }
    }
}
