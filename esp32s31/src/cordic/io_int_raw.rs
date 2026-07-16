#[doc = "Register `IO_INT_RAW` reader"]
pub type R = crate::R<IO_INT_RAW_SPEC>;
#[doc = "Field `CAL_DONE_INT_RAW` reader - The raw interrupt status of CAL_DONE_INT."]
pub type CAL_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `DMA_ALL_CAL_DONE_INT_RAW` reader - The raw interrupt status of DMA_ALL_CAL_DONE_INT."]
pub type DMA_ALL_CAL_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_AFIFO_WOVF_INT_RAW` reader - The raw interrupt status of OUT_AFIFO_WOVF_INT."]
pub type OUT_AFIFO_WOVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_AFIFO_RUDF_INT_RAW` reader - The raw interrupt status of IN_AFIFO_RUDF_INT."]
pub type IN_AFIFO_RUDF_INT_RAW_R = crate::BitReader;
#[doc = "Field `ARG_NUM_ERR_INT_RAW` reader - The raw interrupt status of ARG_NUM_ERR_INT."]
pub type ARG_NUM_ERR_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of CAL_DONE_INT."]
    #[inline(always)]
    pub fn cal_done_int_raw(&self) -> CAL_DONE_INT_RAW_R {
        CAL_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of DMA_ALL_CAL_DONE_INT."]
    #[inline(always)]
    pub fn dma_all_cal_done_int_raw(&self) -> DMA_ALL_CAL_DONE_INT_RAW_R {
        DMA_ALL_CAL_DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of OUT_AFIFO_WOVF_INT."]
    #[inline(always)]
    pub fn out_afifo_wovf_int_raw(&self) -> OUT_AFIFO_WOVF_INT_RAW_R {
        OUT_AFIFO_WOVF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of IN_AFIFO_RUDF_INT."]
    #[inline(always)]
    pub fn in_afifo_rudf_int_raw(&self) -> IN_AFIFO_RUDF_INT_RAW_R {
        IN_AFIFO_RUDF_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of ARG_NUM_ERR_INT."]
    #[inline(always)]
    pub fn arg_num_err_int_raw(&self) -> ARG_NUM_ERR_INT_RAW_R {
        ARG_NUM_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_INT_RAW")
            .field("cal_done_int_raw", &self.cal_done_int_raw())
            .field("dma_all_cal_done_int_raw", &self.dma_all_cal_done_int_raw())
            .field("out_afifo_wovf_int_raw", &self.out_afifo_wovf_int_raw())
            .field("in_afifo_rudf_int_raw", &self.in_afifo_rudf_int_raw())
            .field("arg_num_err_int_raw", &self.arg_num_err_int_raw())
            .finish()
    }
}
#[doc = "Cordic interrupt raw singal status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`io_int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_INT_RAW_SPEC;
impl crate::RegisterSpec for IO_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_int_raw::R`](R) reader structure"]
impl crate::Readable for IO_INT_RAW_SPEC {}
#[doc = "`reset()` method sets IO_INT_RAW to value 0"]
impl crate::Resettable for IO_INT_RAW_SPEC {}
