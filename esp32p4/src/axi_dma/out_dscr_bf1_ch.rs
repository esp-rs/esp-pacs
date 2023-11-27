#[doc = "Register `OUT_DSCR_BF1_CH%s` reader"]
pub type R = crate::R<OUT_DSCR_BF1_CH_SPEC>;
#[doc = "Field `OUTLINK_DSCR_BF1_CH` reader - The address of the second-to-last outlink descriptor x-2."]
pub type OUTLINK_DSCR_BF1_CH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the second-to-last outlink descriptor x-2."]
    #[inline(always)]
    pub fn outlink_dscr_bf1_ch(&self) -> OUTLINK_DSCR_BF1_CH_R {
        OUTLINK_DSCR_BF1_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_DSCR_BF1_CH")
            .field(
                "outlink_dscr_bf1_ch",
                &format_args!("{}", self.outlink_dscr_bf1_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_DSCR_BF1_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "The second-to-last outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_DSCR_BF1_CH_SPEC;
impl crate::RegisterSpec for OUT_DSCR_BF1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_dscr_bf1_ch::R`](R) reader structure"]
impl crate::Readable for OUT_DSCR_BF1_CH_SPEC {}
#[doc = "`reset()` method sets OUT_DSCR_BF1_CH%s to value 0"]
impl crate::Resettable for OUT_DSCR_BF1_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
