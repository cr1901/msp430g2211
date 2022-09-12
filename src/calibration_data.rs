#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCOCTL Calibration Data for 1MHz"]
    pub caldco_1mhz: CALDCO_1MHZ,
    #[doc = "0x01 - BCSCTL1 Calibration Data for 1MHz"]
    pub calbc1_1mhz: CALBC1_1MHZ,
}
#[doc = "CALDCO_1MHZ (rw) register accessor: an alias for `Reg<CALDCO_1MHZ_SPEC>`"]
pub type CALDCO_1MHZ = crate::Reg<caldco_1mhz::CALDCO_1MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 1MHz"]
pub mod caldco_1mhz;
#[doc = "CALBC1_1MHZ (rw) register accessor: an alias for `Reg<CALBC1_1MHZ_SPEC>`"]
pub type CALBC1_1MHZ = crate::Reg<calbc1_1mhz::CALBC1_1MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 1MHz"]
pub mod calbc1_1mhz;
