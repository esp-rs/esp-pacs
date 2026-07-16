#[doc = "Register `IO_INT_ENA` reader"]
pub type R = crate::R<IO_INT_ENA_SPEC>;
#[doc = "Register `IO_INT_ENA` writer"]
pub type W = crate::W<IO_INT_ENA_SPEC>;
#[doc = "Field `CAL_DONE_INT_ENA` reader - Write 1 to enable CAL_DONE_INT."]
pub type CAL_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `CAL_DONE_INT_ENA` writer - Write 1 to enable CAL_DONE_INT."]
pub type CAL_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_ALL_CAL_DONE_INT_ENA` reader - Write 1 to enable DMA_ALL_CAL_DONE_INT."]
pub type DMA_ALL_CAL_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `DMA_ALL_CAL_DONE_INT_ENA` writer - Write 1 to enable DMA_ALL_CAL_DONE_INT."]
pub type DMA_ALL_CAL_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AFIFO_WOVF_INT_ENA` reader - Write 1 to enable OUT_AFIFO_WOVF_INT."]
pub type OUT_AFIFO_WOVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_AFIFO_WOVF_INT_ENA` writer - Write 1 to enable OUT_AFIFO_WOVF_INT."]
pub type OUT_AFIFO_WOVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_AFIFO_RUDF_INT_ENA` reader - Write 1 to enable IN_AFIFO_RUDF_INT."]
pub type IN_AFIFO_RUDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_AFIFO_RUDF_INT_ENA` writer - Write 1 to enable IN_AFIFO_RUDF_INT."]
pub type IN_AFIFO_RUDF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARG_NUM_ERR_INT_ENA` reader - Write 1 to enable ARG_NUM_ERR_INT."]
pub type ARG_NUM_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `ARG_NUM_ERR_INT_ENA` writer - Write 1 to enable ARG_NUM_ERR_INT."]
pub type ARG_NUM_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable CAL_DONE_INT."]
    #[inline(always)]
    pub fn cal_done_int_ena(&self) -> CAL_DONE_INT_ENA_R {
        CAL_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable DMA_ALL_CAL_DONE_INT."]
    #[inline(always)]
    pub fn dma_all_cal_done_int_ena(&self) -> DMA_ALL_CAL_DONE_INT_ENA_R {
        DMA_ALL_CAL_DONE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable OUT_AFIFO_WOVF_INT."]
    #[inline(always)]
    pub fn out_afifo_wovf_int_ena(&self) -> OUT_AFIFO_WOVF_INT_ENA_R {
        OUT_AFIFO_WOVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable IN_AFIFO_RUDF_INT."]
    #[inline(always)]
    pub fn in_afifo_rudf_int_ena(&self) -> IN_AFIFO_RUDF_INT_ENA_R {
        IN_AFIFO_RUDF_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable ARG_NUM_ERR_INT."]
    #[inline(always)]
    pub fn arg_num_err_int_ena(&self) -> ARG_NUM_ERR_INT_ENA_R {
        ARG_NUM_ERR_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_INT_ENA")
            .field("cal_done_int_ena", &self.cal_done_int_ena())
            .field("dma_all_cal_done_int_ena", &self.dma_all_cal_done_int_ena())
            .field("out_afifo_wovf_int_ena", &self.out_afifo_wovf_int_ena())
            .field("in_afifo_rudf_int_ena", &self.in_afifo_rudf_int_ena())
            .field("arg_num_err_int_ena", &self.arg_num_err_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable CAL_DONE_INT."]
    #[inline(always)]
    pub fn cal_done_int_ena(&mut self) -> CAL_DONE_INT_ENA_W<'_, IO_INT_ENA_SPEC> {
        CAL_DONE_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable DMA_ALL_CAL_DONE_INT."]
    #[inline(always)]
    pub fn dma_all_cal_done_int_ena(&mut self) -> DMA_ALL_CAL_DONE_INT_ENA_W<'_, IO_INT_ENA_SPEC> {
        DMA_ALL_CAL_DONE_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable OUT_AFIFO_WOVF_INT."]
    #[inline(always)]
    pub fn out_afifo_wovf_int_ena(&mut self) -> OUT_AFIFO_WOVF_INT_ENA_W<'_, IO_INT_ENA_SPEC> {
        OUT_AFIFO_WOVF_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable IN_AFIFO_RUDF_INT."]
    #[inline(always)]
    pub fn in_afifo_rudf_int_ena(&mut self) -> IN_AFIFO_RUDF_INT_ENA_W<'_, IO_INT_ENA_SPEC> {
        IN_AFIFO_RUDF_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable ARG_NUM_ERR_INT."]
    #[inline(always)]
    pub fn arg_num_err_int_ena(&mut self) -> ARG_NUM_ERR_INT_ENA_W<'_, IO_INT_ENA_SPEC> {
        ARG_NUM_ERR_INT_ENA_W::new(self, 4)
    }
}
#[doc = "Cordic interrupt enable singal configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`io_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_INT_ENA_SPEC;
impl crate::RegisterSpec for IO_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_int_ena::R`](R) reader structure"]
impl crate::Readable for IO_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`io_int_ena::W`](W) writer structure"]
impl crate::Writable for IO_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_INT_ENA to value 0"]
impl crate::Resettable for IO_INT_ENA_SPEC {}
