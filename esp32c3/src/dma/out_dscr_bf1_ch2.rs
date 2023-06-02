#[doc = "Register `OUT_DSCR_BF1_CH2` reader"]
pub struct R(crate::R<OUT_DSCR_BF1_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_DSCR_BF1_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_DSCR_BF1_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_DSCR_BF1_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_BF1` reader - The address of the second-to-last inlink descriptor x-2."]
pub type OUTLINK_DSCR_BF1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the second-to-last inlink descriptor x-2."]
    #[inline(always)]
    pub fn outlink_dscr_bf1(&self) -> OUTLINK_DSCR_BF1_R {
        OUTLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_DSCR_BF1_CH2")
            .field(
                "outlink_dscr_bf1",
                &format_args!("{}", self.outlink_dscr_bf1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_DSCR_BF1_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA_OUT_DSCR_BF1_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_dscr_bf1_ch2](index.html) module"]
pub struct OUT_DSCR_BF1_CH2_SPEC;
impl crate::RegisterSpec for OUT_DSCR_BF1_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_dscr_bf1_ch2::R](R) reader structure"]
impl crate::Readable for OUT_DSCR_BF1_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_DSCR_BF1_CH2 to value 0"]
impl crate::Resettable for OUT_DSCR_BF1_CH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
