#[doc = "Register `INLINK_DSCR_BF1` reader"]
pub struct R(crate::R<INLINK_DSCR_BF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INLINK_DSCR_BF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INLINK_DSCR_BF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INLINK_DSCR_BF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_BF1` reader - The address of next inlink data buffer."]
pub type INLINK_DSCR_BF1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of next inlink data buffer."]
    #[inline(always)]
    pub fn inlink_dscr_bf1(&self) -> INLINK_DSCR_BF1_R {
        INLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[doc = "Address of next inlink data buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inlink_dscr_bf1](index.html) module"]
pub struct INLINK_DSCR_BF1_SPEC;
impl crate::RegisterSpec for INLINK_DSCR_BF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inlink_dscr_bf1::R](R) reader structure"]
impl crate::Readable for INLINK_DSCR_BF1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INLINK_DSCR_BF1 to value 0"]
impl crate::Resettable for INLINK_DSCR_BF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
