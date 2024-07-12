#[doc = "Register `BLK_TFR_RESUMEREQ0` writer"]
pub type W = crate::W<BLK_TFR_RESUMEREQ0_SPEC>;
#[doc = "Field `CH1_BLK_TFR_RESUMEREQ` writer - NA"]
pub type CH1_BLK_TFR_RESUMEREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK_TFR_RESUMEREQ0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_blk_tfr_resumereq(&mut self) -> CH1_BLK_TFR_RESUMEREQ_W<BLK_TFR_RESUMEREQ0_SPEC> {
        CH1_BLK_TFR_RESUMEREQ_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_tfr_resumereq0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK_TFR_RESUMEREQ0_SPEC;
impl crate::RegisterSpec for BLK_TFR_RESUMEREQ0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`blk_tfr_resumereq0::W`](W) writer structure"]
impl crate::Writable for BLK_TFR_RESUMEREQ0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK_TFR_RESUMEREQ0 to value 0"]
impl crate::Resettable for BLK_TFR_RESUMEREQ0_SPEC {
    const RESET_VALUE: u32 = 0;
}
