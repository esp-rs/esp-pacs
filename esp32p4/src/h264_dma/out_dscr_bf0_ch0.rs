#[doc = "Register `OUT_DSCR_BF0_CH0` reader"]
pub type R = crate::R<OUT_DSCR_BF0_CH0_SPEC>;
#[doc = "Field `OUTLINK_DSCR_BF0_CH0` reader - The address of the last outlink descriptor's next address y-1."]
pub type OUTLINK_DSCR_BF0_CH0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the last outlink descriptor's next address y-1."]
    #[inline(always)]
    pub fn outlink_dscr_bf0_ch0(&self) -> OUTLINK_DSCR_BF0_CH0_R {
        OUTLINK_DSCR_BF0_CH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_DSCR_BF0_CH0")
            .field(
                "outlink_dscr_bf0_ch0",
                &format_args!("{}", self.outlink_dscr_bf0_ch0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_DSCR_BF0_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TX CH0 last dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_DSCR_BF0_CH0_SPEC;
impl crate::RegisterSpec for OUT_DSCR_BF0_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_dscr_bf0_ch0::R`](R) reader structure"]
impl crate::Readable for OUT_DSCR_BF0_CH0_SPEC {}
#[doc = "`reset()` method sets OUT_DSCR_BF0_CH0 to value 0"]
impl crate::Resettable for OUT_DSCR_BF0_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
