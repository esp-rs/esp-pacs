#[doc = "Register `MULT_INT_CLR` writer"]
pub type W = crate::W<MULT_INT_CLR_SPEC>;
#[doc = "Field `CALC_DONE_INT_CLR` writer - Set this bit to clear the i2s_rx_done_int interrupt"]
pub type CALC_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MULT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the i2s_rx_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn calc_done_int_clr(&mut self) -> CALC_DONE_INT_CLR_W<MULT_INT_CLR_SPEC> {
        CALC_DONE_INT_CLR_W::new(self, 0)
    }
}
#[doc = "I2S interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_INT_CLR_SPEC;
impl crate::RegisterSpec for MULT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mult_int_clr::W`](W) writer structure"]
impl crate::Writable for MULT_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MULT_INT_CLR to value 0"]
impl crate::Resettable for MULT_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
