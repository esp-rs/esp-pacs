#[doc = "Register `OUT_DSCR_BF0_CH%s` reader"]
pub struct R(crate::R<OUT_DSCR_BF0_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_DSCR_BF0_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_DSCR_BF0_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_DSCR_BF0_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_BF0_CH` reader - The address of the last outlink descriptor y-1."]
pub type OUTLINK_DSCR_BF0_CH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the last outlink descriptor y-1."]
    #[inline(always)]
    pub fn outlink_dscr_bf0_ch(&self) -> OUTLINK_DSCR_BF0_CH_R {
        OUTLINK_DSCR_BF0_CH_R::new(self.bits)
    }
}
#[doc = "The last inlink descriptor address of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_dscr_bf0_ch](index.html) module"]
pub struct OUT_DSCR_BF0_CH_SPEC;
impl crate::RegisterSpec for OUT_DSCR_BF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_dscr_bf0_ch::R](R) reader structure"]
impl crate::Readable for OUT_DSCR_BF0_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_DSCR_BF0_CH%s to value 0"]
impl crate::Resettable for OUT_DSCR_BF0_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
