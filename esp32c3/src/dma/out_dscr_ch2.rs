#[doc = "Register `OUT_DSCR_CH2` reader"]
pub struct R(crate::R<OUT_DSCR_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_DSCR_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_DSCR_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_DSCR_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_CH2` reader - The address of the current outlink descriptor y."]
pub type OUTLINK_DSCR_CH2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the current outlink descriptor y."]
    #[inline(always)]
    pub fn outlink_dscr_ch2(&self) -> OUTLINK_DSCR_CH2_R {
        OUTLINK_DSCR_CH2_R::new(self.bits)
    }
}
#[doc = "DMA_OUT_DSCR_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_dscr_ch2](index.html) module"]
pub struct OUT_DSCR_CH2_SPEC;
impl crate::RegisterSpec for OUT_DSCR_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_dscr_ch2::R](R) reader structure"]
impl crate::Readable for OUT_DSCR_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_DSCR_CH2 to value 0"]
impl crate::Resettable for OUT_DSCR_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
