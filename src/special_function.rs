#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    pub ie1: IE1,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Interrupt Flag 1"]
    pub ifg1: IFG1,
}
#[doc = "IE1 (rw) register accessor: an alias for `Reg<IE1_SPEC>`"]
pub type IE1 = crate::Reg<ie1::IE1_SPEC>;
#[doc = "Interrupt Enable 1"]
pub mod ie1;
#[doc = "IFG1 (rw) register accessor: an alias for `Reg<IFG1_SPEC>`"]
pub type IFG1 = crate::Reg<ifg1::IFG1_SPEC>;
#[doc = "Interrupt Flag 1"]
pub mod ifg1;
