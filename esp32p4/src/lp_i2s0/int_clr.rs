#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `RX_DONE` writer - Set this bit to clear the i2s_rx_done_int interrupt"]
pub type RX_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_HUNG` writer - Set this bit to clear the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_FIFOMEM_UDF` writer - Set this bit to clear the i2s_rx_fifomem_udf_int interrupt"]
pub type RX_FIFOMEM_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LP_VAD_DONE` writer - Set this bit to clear the vad_done_int interrupt"]
pub type LP_VAD_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LP_VAD_RESET_DONE` writer - Set this bit to clear the vad_reset_done_int interrupt"]
pub type LP_VAD_RESET_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_MEM_THRESHOLD` writer - Set this bit to clear the rx_mem_threshold_int interrupt"]
pub type RX_MEM_THRESHOLD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the i2s_rx_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done(&mut self) -> RX_DONE_W<INT_CLR_SPEC> {
        RX_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_CLR_SPEC> {
        RX_HUNG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifomem_udf(&mut self) -> RX_FIFOMEM_UDF_W<INT_CLR_SPEC> {
        RX_FIFOMEM_UDF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the vad_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lp_vad_done(&mut self) -> LP_VAD_DONE_W<INT_CLR_SPEC> {
        LP_VAD_DONE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the vad_reset_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lp_vad_reset_done(&mut self) -> LP_VAD_RESET_DONE_W<INT_CLR_SPEC> {
        LP_VAD_RESET_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mem_threshold(&mut self) -> RX_MEM_THRESHOLD_W<INT_CLR_SPEC> {
        RX_MEM_THRESHOLD_W::new(self, 5)
    }
}
#[doc = "I2S interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
