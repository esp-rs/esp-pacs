#[doc = "Register `OUT_RESET_AVAIL_CH%s` reader"]
pub type R = crate::R<OUT_RESET_AVAIL_CH_SPEC>;
#[doc = "Field `OUT_RESET_AVAIL` reader - tx chan0 reset valid reg."]
pub type OUT_RESET_AVAIL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx chan0 reset valid reg."]
    #[inline(always)]
    pub fn out_reset_avail(&self) -> OUT_RESET_AVAIL_R {
        OUT_RESET_AVAIL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_RESET_AVAIL_CH")
            .field("out_reset_avail", &self.out_reset_avail())
            .finish()
    }
}
#[doc = "The tx channel 0 reset valid_flag register.\n\nYou can [`read`](crate::Reg::read) this register and get [`out_reset_avail_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_RESET_AVAIL_CH_SPEC;
impl crate::RegisterSpec for OUT_RESET_AVAIL_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_reset_avail_ch::R`](R) reader structure"]
impl crate::Readable for OUT_RESET_AVAIL_CH_SPEC {}
#[doc = "`reset()` method sets OUT_RESET_AVAIL_CH%s to value 0x01"]
impl crate::Resettable for OUT_RESET_AVAIL_CH_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
