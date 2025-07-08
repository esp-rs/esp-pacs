#[doc = "Register `IN_INT_CLR_CH1` writer"]
pub type W = crate::W<IN_INT_CLR_CH1_SPEC>;
#[doc = "Field `IN_DONE_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_IN_DONE_CH1_INT"]
pub type IN_DONE_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_IN_SUC_EOF_CH1_INT"]
pub type IN_SUC_EOF_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_IN_ERR_EOF_CH1_INT"]
pub type IN_ERR_EOF_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_IN_DSCR_ERR_CH1_INT"]
pub type IN_DSCR_ERR_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_IN_DSCR_EMPTY_CH1_INT"]
pub type IN_DSCR_EMPTY_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_INFIFO_OVF_CH1_INT"]
pub type INFIFO_OVF_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_INFIFO_UDF_CH1_INT"]
pub type INFIFO_UDF_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_AHBINF_RESP_ERR_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_IN_RESP_ERR_CH1_INT"]
pub type IN_AHBINF_RESP_ERR_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_CLR_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear AHB_DMA_IN_DONE_CH1_INT"]
    #[inline(always)]
    pub fn in_done_ch1_int_clr(&mut self) -> IN_DONE_CH1_INT_CLR_W<IN_INT_CLR_CH1_SPEC> {
        IN_DONE_CH1_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear AHB_DMA_IN_SUC_EOF_CH1_INT"]
    #[inline(always)]
    pub fn in_suc_eof_ch1_int_clr(&mut self) -> IN_SUC_EOF_CH1_INT_CLR_W<IN_INT_CLR_CH1_SPEC> {
        IN_SUC_EOF_CH1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear AHB_DMA_IN_ERR_EOF_CH1_INT"]
    #[inline(always)]
    pub fn in_err_eof_ch1_int_clr(&mut self) -> IN_ERR_EOF_CH1_INT_CLR_W<IN_INT_CLR_CH1_SPEC> {
        IN_ERR_EOF_CH1_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear AHB_DMA_IN_DSCR_ERR_CH1_INT"]
    #[inline(always)]
    pub fn in_dscr_err_ch1_int_clr(&mut self) -> IN_DSCR_ERR_CH1_INT_CLR_W<IN_INT_CLR_CH1_SPEC> {
        IN_DSCR_ERR_CH1_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear AHB_DMA_IN_DSCR_EMPTY_CH1_INT"]
    #[inline(always)]
    pub fn in_dscr_empty_ch1_int_clr(
        &mut self,
    ) -> IN_DSCR_EMPTY_CH1_INT_CLR_W<IN_INT_CLR_CH1_SPEC> {
        IN_DSCR_EMPTY_CH1_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear AHB_DMA_INFIFO_OVF_CH1_INT"]
    #[inline(always)]
    pub fn infifo_ovf_ch1_int_clr(&mut self) -> INFIFO_OVF_CH1_INT_CLR_W<IN_INT_CLR_CH1_SPEC> {
        INFIFO_OVF_CH1_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear AHB_DMA_INFIFO_UDF_CH1_INT"]
    #[inline(always)]
    pub fn infifo_udf_ch1_int_clr(&mut self) -> INFIFO_UDF_CH1_INT_CLR_W<IN_INT_CLR_CH1_SPEC> {
        INFIFO_UDF_CH1_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to clear AHB_DMA_IN_RESP_ERR_CH1_INT"]
    #[inline(always)]
    pub fn in_ahbinf_resp_err_ch1_int_clr(
        &mut self,
    ) -> IN_AHBINF_RESP_ERR_CH1_INT_CLR_W<IN_INT_CLR_CH1_SPEC> {
        IN_AHBINF_RESP_ERR_CH1_INT_CLR_W::new(self, 7)
    }
}
#[doc = "Interrupt clear bits of RX channel 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_clr_ch1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_CLR_CH1_SPEC;
impl crate::RegisterSpec for IN_INT_CLR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`in_int_clr_ch1::W`](W) writer structure"]
impl crate::Writable for IN_INT_CLR_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_INT_CLR_CH1 to value 0"]
impl crate::Resettable for IN_INT_CLR_CH1_SPEC {}
