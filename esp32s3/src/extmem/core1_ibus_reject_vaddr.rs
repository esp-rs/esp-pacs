#[doc = "Register `CORE1_IBUS_REJECT_VADDR` reader"]
pub struct R(crate::R<CORE1_IBUS_REJECT_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE1_IBUS_REJECT_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE1_IBUS_REJECT_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE1_IBUS_REJECT_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE1_IBUS_VADDR` reader - The bits are used to indicate the virtual address of CPU access ibus when authentication fail."]
pub type CORE1_IBUS_VADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address of CPU access ibus when authentication fail."]
    #[inline(always)]
    pub fn core1_ibus_vaddr(&self) -> CORE1_IBUS_VADDR_R {
        CORE1_IBUS_VADDR_R::new(self.bits)
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core1_ibus_reject_vaddr](index.html) module"]
pub struct CORE1_IBUS_REJECT_VADDR_SPEC;
impl crate::RegisterSpec for CORE1_IBUS_REJECT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core1_ibus_reject_vaddr::R](R) reader structure"]
impl crate::Readable for CORE1_IBUS_REJECT_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE1_IBUS_REJECT_VADDR to value 0xffff_ffff"]
impl crate::Resettable for CORE1_IBUS_REJECT_VADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
