#[doc = "Register `IN_DSCR_CH%s` reader"]
pub struct R(crate::R<IN_DSCR_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_DSCR_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_DSCR_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_DSCR_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR` reader - The address of the current inlink descriptor x."]
pub type INLINK_DSCR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the current inlink descriptor x."]
    #[inline(always)]
    pub fn inlink_dscr(&self) -> INLINK_DSCR_R {
        INLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DSCR_CH")
            .field(
                "inlink_dscr",
                &format_args!("{}", self.inlink_dscr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_DSCR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Current inlink descriptor address of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_dscr_ch](index.html) module"]
pub struct IN_DSCR_CH_SPEC;
impl crate::RegisterSpec for IN_DSCR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_dscr_ch::R](R) reader structure"]
impl crate::Readable for IN_DSCR_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_DSCR_CH%s to value 0"]
impl crate::Resettable for IN_DSCR_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
