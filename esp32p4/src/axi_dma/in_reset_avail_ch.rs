#[doc = "Register `IN_RESET_AVAIL_CH%s` reader"]
pub type R = crate::R<IN_RESET_AVAIL_CH_SPEC>;
#[doc = "Field `IN_RESET_AVAIL_CH` reader - rx chan0 reset valid reg."]
pub type IN_RESET_AVAIL_CH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - rx chan0 reset valid reg."]
    #[inline(always)]
    pub fn in_reset_avail_ch(&self) -> IN_RESET_AVAIL_CH_R {
        IN_RESET_AVAIL_CH_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_RESET_AVAIL_CH")
            .field(
                "in_reset_avail_ch",
                &format_args!("{}", self.in_reset_avail_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_RESET_AVAIL_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "The rx channel 0 reset valid_flag register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_reset_avail_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_RESET_AVAIL_CH_SPEC;
impl crate::RegisterSpec for IN_RESET_AVAIL_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_reset_avail_ch::R`](R) reader structure"]
impl crate::Readable for IN_RESET_AVAIL_CH_SPEC {}
#[doc = "`reset()` method sets IN_RESET_AVAIL_CH%s to value 0x01"]
impl crate::Resettable for IN_RESET_AVAIL_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
