#[doc = "Register `IO_INT_CLR` writer"]
pub type W = crate::W<IO_INT_CLR_SPEC>;
#[doc = "Field `CAL_DONE_INT_CLR` writer - Write 1 to clear CAL_DONE_INT."]
pub type CAL_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_ALL_CAL_DONE_INT_CLR` writer - Write 1 to clear DMA_ALL_CAL_DONE_INT."]
pub type DMA_ALL_CAL_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AFIFO_WOVF_INT_CLR` writer - Write 1 to clear OUT_AFIFO_WOVF_INT."]
pub type OUT_AFIFO_WOVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_AFIFO_RUDF_INT_CLR` writer - Write 1 to clear IN_AFIFO_RUDF_INT."]
pub type IN_AFIFO_RUDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARG_NUM_ERR_INT_CLR` writer - Write 1 to clear ARG_NUM_ERR_INT."]
pub type ARG_NUM_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IO_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear CAL_DONE_INT."]
    #[inline(always)]
    pub fn cal_done_int_clr(&mut self) -> CAL_DONE_INT_CLR_W<'_, IO_INT_CLR_SPEC> {
        CAL_DONE_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear DMA_ALL_CAL_DONE_INT."]
    #[inline(always)]
    pub fn dma_all_cal_done_int_clr(&mut self) -> DMA_ALL_CAL_DONE_INT_CLR_W<'_, IO_INT_CLR_SPEC> {
        DMA_ALL_CAL_DONE_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear OUT_AFIFO_WOVF_INT."]
    #[inline(always)]
    pub fn out_afifo_wovf_int_clr(&mut self) -> OUT_AFIFO_WOVF_INT_CLR_W<'_, IO_INT_CLR_SPEC> {
        OUT_AFIFO_WOVF_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear IN_AFIFO_RUDF_INT."]
    #[inline(always)]
    pub fn in_afifo_rudf_int_clr(&mut self) -> IN_AFIFO_RUDF_INT_CLR_W<'_, IO_INT_CLR_SPEC> {
        IN_AFIFO_RUDF_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear ARG_NUM_ERR_INT."]
    #[inline(always)]
    pub fn arg_num_err_int_clr(&mut self) -> ARG_NUM_ERR_INT_CLR_W<'_, IO_INT_CLR_SPEC> {
        ARG_NUM_ERR_INT_CLR_W::new(self, 4)
    }
}
#[doc = "Cordic interrupt clear singal configuration register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_INT_CLR_SPEC;
impl crate::RegisterSpec for IO_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`io_int_clr::W`](W) writer structure"]
impl crate::Writable for IO_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_INT_CLR to value 0"]
impl crate::Resettable for IO_INT_CLR_SPEC {}
