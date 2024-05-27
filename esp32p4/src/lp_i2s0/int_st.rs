#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `RX_DONE` reader - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_R = crate::BitReader;
#[doc = "Field `RX_HUNG` reader - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_R = crate::BitReader;
#[doc = "Field `RX_FIFOMEM_UDF` reader - The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RX_FIFOMEM_UDF_R = crate::BitReader;
#[doc = "Field `LP_VAD_DONE` reader - The masked interrupt status bit for the vad_done_int interrupt"]
pub type LP_VAD_DONE_R = crate::BitReader;
#[doc = "Field `LP_VAD_RESET_DONE` reader - The masked interrupt status bit for the vad_reset_done_int interrupt"]
pub type LP_VAD_RESET_DONE_R = crate::BitReader;
#[doc = "Field `RX_MEM_THRESHOLD` reader - The masked interrupt status bit for the rx_mem_threshold_int interrupt"]
pub type RX_MEM_THRESHOLD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf(&self) -> RX_FIFOMEM_UDF_R {
        RX_FIFOMEM_UDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_done(&self) -> LP_VAD_DONE_R {
        LP_VAD_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_reset_done(&self) -> LP_VAD_RESET_DONE_R {
        LP_VAD_RESET_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold(&self) -> RX_MEM_THRESHOLD_R {
        RX_MEM_THRESHOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("rx_done", &self.rx_done())
            .field("rx_hung", &self.rx_hung())
            .field("rx_fifomem_udf", &self.rx_fifomem_udf())
            .field("lp_vad_done", &self.lp_vad_done())
            .field("lp_vad_reset_done", &self.lp_vad_reset_done())
            .field("rx_mem_threshold", &self.rx_mem_threshold())
            .finish()
    }
}
#[doc = "I2S interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
