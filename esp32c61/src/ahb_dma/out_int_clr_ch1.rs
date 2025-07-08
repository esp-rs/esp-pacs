#[doc = "Register `OUT_INT_CLR_CH1` writer"]
pub type W = crate::W<OUT_INT_CLR_CH1_SPEC>;
#[doc = "Field `OUT_DONE_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_DONE_CH1_INT"]
pub type OUT_DONE_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_EOF_CH1_INT"]
pub type OUT_EOF_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_DSCR_ERR_CH1_INT"]
pub type OUT_DSCR_ERR_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_TOTAL_EOF_CH1_INT"]
pub type OUT_TOTAL_EOF_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_OUTFIFO_OVF_CH1_INT"]
pub type OUTFIFO_OVF_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_OUTFIFO_UDF_CH1_INT"]
pub type OUTFIFO_UDF_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AHBINF_RESP_ERR_CH1_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_RESP_ERR_CH1_INT"]
pub type OUT_AHBINF_RESP_ERR_CH1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_INT_CLR_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear AHB_DMA_OUT_DONE_CH1_INT"]
    #[inline(always)]
    pub fn out_done_ch1_int_clr(&mut self) -> OUT_DONE_CH1_INT_CLR_W<OUT_INT_CLR_CH1_SPEC> {
        OUT_DONE_CH1_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear AHB_DMA_OUT_EOF_CH1_INT"]
    #[inline(always)]
    pub fn out_eof_ch1_int_clr(&mut self) -> OUT_EOF_CH1_INT_CLR_W<OUT_INT_CLR_CH1_SPEC> {
        OUT_EOF_CH1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear AHB_DMA_OUT_DSCR_ERR_CH1_INT"]
    #[inline(always)]
    pub fn out_dscr_err_ch1_int_clr(&mut self) -> OUT_DSCR_ERR_CH1_INT_CLR_W<OUT_INT_CLR_CH1_SPEC> {
        OUT_DSCR_ERR_CH1_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear AHB_DMA_OUT_TOTAL_EOF_CH1_INT"]
    #[inline(always)]
    pub fn out_total_eof_ch1_int_clr(
        &mut self,
    ) -> OUT_TOTAL_EOF_CH1_INT_CLR_W<OUT_INT_CLR_CH1_SPEC> {
        OUT_TOTAL_EOF_CH1_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear AHB_DMA_OUTFIFO_OVF_CH1_INT"]
    #[inline(always)]
    pub fn outfifo_ovf_ch1_int_clr(&mut self) -> OUTFIFO_OVF_CH1_INT_CLR_W<OUT_INT_CLR_CH1_SPEC> {
        OUTFIFO_OVF_CH1_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear AHB_DMA_OUTFIFO_UDF_CH1_INT"]
    #[inline(always)]
    pub fn outfifo_udf_ch1_int_clr(&mut self) -> OUTFIFO_UDF_CH1_INT_CLR_W<OUT_INT_CLR_CH1_SPEC> {
        OUTFIFO_UDF_CH1_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear AHB_DMA_OUT_RESP_ERR_CH1_INT"]
    #[inline(always)]
    pub fn out_ahbinf_resp_err_ch1_int_clr(
        &mut self,
    ) -> OUT_AHBINF_RESP_ERR_CH1_INT_CLR_W<OUT_INT_CLR_CH1_SPEC> {
        OUT_AHBINF_RESP_ERR_CH1_INT_CLR_W::new(self, 6)
    }
}
#[doc = "Interrupt clear bits of TX channel 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_clr_ch1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_CLR_CH1_SPEC;
impl crate::RegisterSpec for OUT_INT_CLR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_int_clr_ch1::W`](W) writer structure"]
impl crate::Writable for OUT_INT_CLR_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_INT_CLR_CH1 to value 0"]
impl crate::Resettable for OUT_INT_CLR_CH1_SPEC {}
