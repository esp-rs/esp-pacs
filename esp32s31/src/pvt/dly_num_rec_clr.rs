#[doc = "Register `DLY_NUM_REC_CLR` writer"]
pub type W = crate::W<DLY_NUM_REC_CLR_SPEC>;
#[doc = "Field `DELAY_NUM_REC_CLR` writer - needs field desc"]
pub type DELAY_NUM_REC_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DLY_NUM_REC_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn delay_num_rec_clr(&mut self) -> DELAY_NUM_REC_CLR_W<'_, DLY_NUM_REC_CLR_SPEC> {
        DELAY_NUM_REC_CLR_W::new(self, 0)
    }
}
#[doc = "needs field desc\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly_num_rec_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLY_NUM_REC_CLR_SPEC;
impl crate::RegisterSpec for DLY_NUM_REC_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dly_num_rec_clr::W`](W) writer structure"]
impl crate::Writable for DLY_NUM_REC_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLY_NUM_REC_CLR to value 0"]
impl crate::Resettable for DLY_NUM_REC_CLR_SPEC {}
