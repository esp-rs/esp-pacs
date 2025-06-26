#[doc = "Register `IN_INT_CLR_CH%s` writer"]
pub type W = crate::W<IN_INT_CLR_CH_SPEC>;
#[doc = "Field `IN_DONE` writer - Write 1 to clear AHB_DMA_IN_DONE_CH%s_INT."]
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` writer - Write 1 to clear AHB_DMA_IN_SUC_EOF_CH%s_INT."]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` writer - Write 1 to clear AHB_DMA_IN_ERR_EOF_CH%s_INT."]
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` writer - Write 1 to clear AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` writer - Write 1 to clear AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF` writer - Write 1 to clear AHB_DMA_INFIFO_OVF_CH%s_INT."]
pub type INFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF` writer - Write 1 to clear AHB_DMA_INFIFO_UDF_CH%s_INT."]
pub type INFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_CLR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear AHB_DMA_IN_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn in_done(&mut self) -> IN_DONE_W<IN_INT_CLR_CH_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear AHB_DMA_IN_SUC_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<IN_INT_CLR_CH_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear AHB_DMA_IN_ERR_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<IN_INT_CLR_CH_SPEC> {
        IN_ERR_EOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<IN_INT_CLR_CH_SPEC> {
        IN_DSCR_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<IN_INT_CLR_CH_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear AHB_DMA_INFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_ovf(&mut self) -> INFIFO_OVF_W<IN_INT_CLR_CH_SPEC> {
        INFIFO_OVF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear AHB_DMA_INFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_udf(&mut self) -> INFIFO_UDF_W<IN_INT_CLR_CH_SPEC> {
        INFIFO_UDF_W::new(self, 6)
    }
}
#[doc = "Interrupt clear bits of RX channel %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_clr_ch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_CLR_CH_SPEC;
impl crate::RegisterSpec for IN_INT_CLR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`in_int_clr_ch::W`](W) writer structure"]
impl crate::Writable for IN_INT_CLR_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_INT_CLR_CH%s to value 0"]
impl crate::Resettable for IN_INT_CLR_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
