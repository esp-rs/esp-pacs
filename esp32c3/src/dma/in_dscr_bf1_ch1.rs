#[doc = "Register `IN_DSCR_BF1_CH1` reader"]
pub struct R(crate::R<IN_DSCR_BF1_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_DSCR_BF1_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_DSCR_BF1_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_DSCR_BF1_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_BF1` reader - The address of the second-to-last inlink descriptor x-2."]
pub type INLINK_DSCR_BF1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the second-to-last inlink descriptor x-2."]
    #[inline(always)]
    pub fn inlink_dscr_bf1(&self) -> INLINK_DSCR_BF1_R {
        INLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DSCR_BF1_CH1")
            .field(
                "inlink_dscr_bf1",
                &format_args!("{}", self.inlink_dscr_bf1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_DSCR_BF1_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA_IN_DSCR_BF1_CH1_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_dscr_bf1_ch1](index.html) module"]
pub struct IN_DSCR_BF1_CH1_SPEC;
impl crate::RegisterSpec for IN_DSCR_BF1_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_dscr_bf1_ch1::R](R) reader structure"]
impl crate::Readable for IN_DSCR_BF1_CH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_DSCR_BF1_CH1 to value 0"]
impl crate::Resettable for IN_DSCR_BF1_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
