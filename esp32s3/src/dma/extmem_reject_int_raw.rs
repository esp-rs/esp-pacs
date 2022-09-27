#[doc = "Register `EXTMEM_REJECT_INT_RAW` reader"]
pub struct R(crate::R<EXTMEM_REJECT_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTMEM_REJECT_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTMEM_REJECT_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTMEM_REJECT_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTMEM_REJECT_INT_RAW` reader - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
pub type EXTMEM_REJECT_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
    #[inline(always)]
    pub fn extmem_reject_int_raw(&self) -> EXTMEM_REJECT_INT_RAW_R {
        EXTMEM_REJECT_INT_RAW_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Raw interrupt status of external RAM permission\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_reject_int_raw](index.html) module"]
pub struct EXTMEM_REJECT_INT_RAW_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extmem_reject_int_raw::R](R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_RAW to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
