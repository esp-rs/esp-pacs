#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `RX_DONE_INT_ST` reader - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_ST` reader - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_FIFOMEM_UDF_INT_ST` reader - The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RX_FIFOMEM_UDF_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_VAD_DONE_INT_ST` reader - The masked interrupt status bit for the vad_done_int interrupt"]
pub type LP_VAD_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_VAD_RESET_DONE_INT_ST` reader - The masked interrupt status bit for the vad_reset_done_int interrupt"]
pub type LP_VAD_RESET_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_MEM_THRESHOLD_INT_ST` reader - The masked interrupt status bit for the rx_mem_threshold_int interrupt"]
pub type RX_MEM_THRESHOLD_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done_int_st(&self) -> RX_DONE_INT_ST_R {
        RX_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> RX_HUNG_INT_ST_R {
        RX_HUNG_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf_int_st(&self) -> RX_FIFOMEM_UDF_INT_ST_R {
        RX_FIFOMEM_UDF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_done_int_st(&self) -> LP_VAD_DONE_INT_ST_R {
        LP_VAD_DONE_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_reset_done_int_st(&self) -> LP_VAD_RESET_DONE_INT_ST_R {
        LP_VAD_RESET_DONE_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold_int_st(&self) -> RX_MEM_THRESHOLD_INT_ST_R {
        RX_MEM_THRESHOLD_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "rx_done_int_st",
                &format_args!("{}", self.rx_done_int_st().bit()),
            )
            .field(
                "rx_hung_int_st",
                &format_args!("{}", self.rx_hung_int_st().bit()),
            )
            .field(
                "rx_fifomem_udf_int_st",
                &format_args!("{}", self.rx_fifomem_udf_int_st().bit()),
            )
            .field(
                "lp_vad_done_int_st",
                &format_args!("{}", self.lp_vad_done_int_st().bit()),
            )
            .field(
                "lp_vad_reset_done_int_st",
                &format_args!("{}", self.lp_vad_reset_done_int_st().bit()),
            )
            .field(
                "rx_mem_threshold_int_st",
                &format_args!("{}", self.rx_mem_threshold_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
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
    const RESET_VALUE: Self::Ux = 0;
}
