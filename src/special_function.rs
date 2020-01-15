#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    pub ie1: IE1,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Interrupt Flag 1"]
    pub ifg1: IFG1,
}
#[doc = "Interrupt Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie1](ie1) module"]
pub type IE1 = crate::Reg<u8, _IE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE1;
#[doc = "`read()` method returns [ie1::R](ie1::R) reader structure"]
impl crate::Readable for IE1 {}
#[doc = "`write(|w| ..)` method takes [ie1::W](ie1::W) writer structure"]
impl crate::Writable for IE1 {}
#[doc = "Interrupt Enable 1"]
pub mod ie1;
#[doc = "Interrupt Flag 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifg1](ifg1) module"]
pub type IFG1 = crate::Reg<u8, _IFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFG1;
#[doc = "`read()` method returns [ifg1::R](ifg1::R) reader structure"]
impl crate::Readable for IFG1 {}
#[doc = "`write(|w| ..)` method takes [ifg1::W](ifg1::W) writer structure"]
impl crate::Writable for IFG1 {}
#[doc = "Interrupt Flag 1"]
pub mod ifg1;
