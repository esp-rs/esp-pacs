#[doc = "Register `IO_INT_ST` reader"]
pub type R = crate::R<IO_INT_ST_SPEC>;
#[doc = "Field `CAL_DONE_INT_ST` reader - The masked interrupt status of CAL_DONE_INT."]
pub type CAL_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `DMA_ALL_CAL_DONE_INT_ST` reader - The masked interrupt status of DMA_ALL_CAL_DONE_INT."]
pub type DMA_ALL_CAL_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_AFIFO_WOVF_INT_ST` reader - The masked interrupt status of OUT_AFIFO_WOVF_INT."]
pub type OUT_AFIFO_WOVF_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_AFIFO_RUDF_INT_ST` reader - The masked interrupt status of IN_AFIFO_RUDF_INT."]
pub type IN_AFIFO_RUDF_INT_ST_R = crate::BitReader;
#[doc = "Field `ARG_NUM_ERR_INT_ST` reader - The masked interrupt status of ARG_NUM_ERR_INT."]
pub type ARG_NUM_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of CAL_DONE_INT."]
    #[inline(always)]
    pub fn cal_done_int_st(&self) -> CAL_DONE_INT_ST_R {
        CAL_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of DMA_ALL_CAL_DONE_INT."]
    #[inline(always)]
    pub fn dma_all_cal_done_int_st(&self) -> DMA_ALL_CAL_DONE_INT_ST_R {
        DMA_ALL_CAL_DONE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of OUT_AFIFO_WOVF_INT."]
    #[inline(always)]
    pub fn out_afifo_wovf_int_st(&self) -> OUT_AFIFO_WOVF_INT_ST_R {
        OUT_AFIFO_WOVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status of IN_AFIFO_RUDF_INT."]
    #[inline(always)]
    pub fn in_afifo_rudf_int_st(&self) -> IN_AFIFO_RUDF_INT_ST_R {
        IN_AFIFO_RUDF_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status of ARG_NUM_ERR_INT."]
    #[inline(always)]
    pub fn arg_num_err_int_st(&self) -> ARG_NUM_ERR_INT_ST_R {
        ARG_NUM_ERR_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_INT_ST")
            .field("cal_done_int_st", &self.cal_done_int_st())
            .field("dma_all_cal_done_int_st", &self.dma_all_cal_done_int_st())
            .field("out_afifo_wovf_int_st", &self.out_afifo_wovf_int_st())
            .field("in_afifo_rudf_int_st", &self.in_afifo_rudf_int_st())
            .field("arg_num_err_int_st", &self.arg_num_err_int_st())
            .finish()
    }
}
#[doc = "Cordic interrupt singal status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`io_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_INT_ST_SPEC;
impl crate::RegisterSpec for IO_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_int_st::R`](R) reader structure"]
impl crate::Readable for IO_INT_ST_SPEC {}
#[doc = "`reset()` method sets IO_INT_ST to value 0"]
impl crate::Resettable for IO_INT_ST_SPEC {}
