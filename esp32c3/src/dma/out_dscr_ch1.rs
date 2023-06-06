#[doc = "Register `OUT_DSCR_CH1` reader"]
pub struct R(crate::R<OUT_DSCR_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_DSCR_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_DSCR_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_DSCR_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR` reader - The address of the current outlink descriptor y."]
pub type OUTLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the current outlink descriptor y."]
    #[inline(always)]
    pub fn outlink_dscr(&self) -> OUTLINK_DSCR_R {
        OUTLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_DSCR_CH1")
            .field(
                "outlink_dscr",
                &format_args!("{}", self.outlink_dscr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_DSCR_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA_OUT_DSCR_CH1_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_dscr_ch1](index.html) module"]
pub struct OUT_DSCR_CH1_SPEC;
impl crate::RegisterSpec for OUT_DSCR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_dscr_ch1::R](R) reader structure"]
impl crate::Readable for OUT_DSCR_CH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_DSCR_CH1 to value 0"]
impl crate::Resettable for OUT_DSCR_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
