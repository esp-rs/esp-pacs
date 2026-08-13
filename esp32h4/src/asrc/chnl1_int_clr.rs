#[doc = "Register `CHNL1_INT_CLR` writer"]
pub type W = crate::W<CHNL1_INT_CLR_SPEC>;
#[doc = "Field `CHNL1_OUTCNT_EOF_INT_CLR` writer - Set this bit to clear the reg_out_cnt_eof_int_raw interrupt."]
pub type CHNL1_OUTCNT_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_OUTFIFO_EOF_INT_CLR` writer - Set this bit to clear the reg_outfifo_eof_int_raw interrupt."]
pub type CHNL1_OUTFIFO_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHNL1_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the reg_out_cnt_eof_int_raw interrupt."]
    #[inline(always)]
    pub fn chnl1_outcnt_eof_int_clr(
        &mut self,
    ) -> CHNL1_OUTCNT_EOF_INT_CLR_W<'_, CHNL1_INT_CLR_SPEC> {
        CHNL1_OUTCNT_EOF_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the reg_outfifo_eof_int_raw interrupt."]
    #[inline(always)]
    pub fn chnl1_outfifo_eof_int_clr(
        &mut self,
    ) -> CHNL1_OUTFIFO_EOF_INT_CLR_W<'_, CHNL1_INT_CLR_SPEC> {
        CHNL1_OUTFIFO_EOF_INT_CLR_W::new(self, 1)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_INT_CLR_SPEC;
impl crate::RegisterSpec for CHNL1_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnl1_int_clr::W`](W) writer structure"]
impl crate::Writable for CHNL1_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL1_INT_CLR to value 0"]
impl crate::Resettable for CHNL1_INT_CLR_SPEC {}
