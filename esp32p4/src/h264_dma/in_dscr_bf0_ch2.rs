#[doc = "Register `IN_DSCR_BF0_CH2` reader"]
pub type R = crate::R<IN_DSCR_BF0_CH2_SPEC>;
#[doc = "Field `INLINK_DSCR_BF0_CH2` reader - The address of the last inlink descriptor's next address x-1."]
pub type INLINK_DSCR_BF0_CH2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the last inlink descriptor's next address x-1."]
    #[inline(always)]
    pub fn inlink_dscr_bf0_ch2(&self) -> INLINK_DSCR_BF0_CH2_R {
        INLINK_DSCR_BF0_CH2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DSCR_BF0_CH2")
            .field(
                "inlink_dscr_bf0_ch2",
                &format_args!("{}", self.inlink_dscr_bf0_ch2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_DSCR_BF0_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RX CH2 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_DSCR_BF0_CH2_SPEC;
impl crate::RegisterSpec for IN_DSCR_BF0_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_dscr_bf0_ch2::R`](R) reader structure"]
impl crate::Readable for IN_DSCR_BF0_CH2_SPEC {}
#[doc = "`reset()` method sets IN_DSCR_BF0_CH2 to value 0"]
impl crate::Resettable for IN_DSCR_BF0_CH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
