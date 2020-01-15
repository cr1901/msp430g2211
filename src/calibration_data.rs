#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCOCTL Calibration Data for 1MHz"]
    pub caldco_1mhz: CALDCO_1MHZ,
    #[doc = "0x01 - BCSCTL1 Calibration Data for 1MHz"]
    pub calbc1_1mhz: CALBC1_1MHZ,
}
#[doc = "DCOCTL Calibration Data for 1MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caldco_1mhz](caldco_1mhz) module"]
pub type CALDCO_1MHZ = crate::Reg<u8, _CALDCO_1MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALDCO_1MHZ;
#[doc = "`read()` method returns [caldco_1mhz::R](caldco_1mhz::R) reader structure"]
impl crate::Readable for CALDCO_1MHZ {}
#[doc = "`write(|w| ..)` method takes [caldco_1mhz::W](caldco_1mhz::W) writer structure"]
impl crate::Writable for CALDCO_1MHZ {}
#[doc = "DCOCTL Calibration Data for 1MHz"]
pub mod caldco_1mhz;
#[doc = "BCSCTL1 Calibration Data for 1MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calbc1_1mhz](calbc1_1mhz) module"]
pub type CALBC1_1MHZ = crate::Reg<u8, _CALBC1_1MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALBC1_1MHZ;
#[doc = "`read()` method returns [calbc1_1mhz::R](calbc1_1mhz::R) reader structure"]
impl crate::Readable for CALBC1_1MHZ {}
#[doc = "`write(|w| ..)` method takes [calbc1_1mhz::W](calbc1_1mhz::W) writer structure"]
impl crate::Writable for CALBC1_1MHZ {}
#[doc = "BCSCTL1 Calibration Data for 1MHz"]
pub mod calbc1_1mhz;
