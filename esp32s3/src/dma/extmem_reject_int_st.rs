#[doc = "Register `EXTMEM_REJECT_INT_ST` reader"]
pub struct R(crate::R<EXTMEM_REJECT_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTMEM_REJECT_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTMEM_REJECT_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTMEM_REJECT_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTMEM_REJECT_INT_ST` reader - The raw interrupt status bit for the EXTMEM_REJECT_INT interrupt."]
pub type EXTMEM_REJECT_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the EXTMEM_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn extmem_reject_int_st(&self) -> EXTMEM_REJECT_INT_ST_R {
        EXTMEM_REJECT_INT_ST_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked interrupt status of external RAM permission\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_reject_int_st](index.html) module"]
pub struct EXTMEM_REJECT_INT_ST_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extmem_reject_int_st::R](R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_ST to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
