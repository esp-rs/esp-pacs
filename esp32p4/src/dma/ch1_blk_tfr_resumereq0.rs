#[doc = "Register `CH1_BLK_TFR_RESUMEREQ0` writer"]
pub type W = crate::W<CH1_BLK_TFR_RESUMEREQ0_SPEC>;
#[doc = "Field `CH1_BLK_TFR_RESUMEREQ` writer - NA"]
pub type CH1_BLK_TFR_RESUMEREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH1_BLK_TFR_RESUMEREQ0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_blk_tfr_resumereq(
        &mut self,
    ) -> CH1_BLK_TFR_RESUMEREQ_W<CH1_BLK_TFR_RESUMEREQ0_SPEC> {
        CH1_BLK_TFR_RESUMEREQ_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_blk_tfr_resumereq0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1_BLK_TFR_RESUMEREQ0_SPEC;
impl crate::RegisterSpec for CH1_BLK_TFR_RESUMEREQ0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch1_blk_tfr_resumereq0::W`](W) writer structure"]
impl crate::Writable for CH1_BLK_TFR_RESUMEREQ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1_BLK_TFR_RESUMEREQ0 to value 0"]
impl crate::Resettable for CH1_BLK_TFR_RESUMEREQ0_SPEC {
    const RESET_VALUE: u32 = 0;
}
