#[doc = "Register `OUT_INT_CLR_CH%s` writer"]
pub type W = crate::W<OUT_INT_CLR_CH_SPEC>;
#[doc = "Field `OUT_DONE` writer - Write 1 to clear AHB_DMA_OUT_DONE_CH%s_INT."]
pub type OUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` writer - Write 1 to clear AHB_DMA_OUT_EOF_CH%s_INT."]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` writer - Write 1 to clear AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` writer - Write 1 to clear AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF` writer - Write 1 to clear AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
pub type OUTFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF` writer - Write 1 to clear AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
pub type OUTFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_INT_CLR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear AHB_DMA_OUT_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn out_done(&mut self) -> OUT_DONE_W<OUT_INT_CLR_CH_SPEC> {
        OUT_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear AHB_DMA_OUT_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OUT_EOF_W<OUT_INT_CLR_CH_SPEC> {
        OUT_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<OUT_INT_CLR_CH_SPEC> {
        OUT_DSCR_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<OUT_INT_CLR_CH_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn outfifo_ovf(&mut self) -> OUTFIFO_OVF_W<OUT_INT_CLR_CH_SPEC> {
        OUTFIFO_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn outfifo_udf(&mut self) -> OUTFIFO_UDF_W<OUT_INT_CLR_CH_SPEC> {
        OUTFIFO_UDF_W::new(self, 5)
    }
}
#[doc = "Interrupt clear bits of TX channel %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_clr_ch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_CLR_CH_SPEC;
impl crate::RegisterSpec for OUT_INT_CLR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_int_clr_ch::W`](W) writer structure"]
impl crate::Writable for OUT_INT_CLR_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_INT_CLR_CH%s to value 0"]
impl crate::Resettable for OUT_INT_CLR_CH_SPEC {}
