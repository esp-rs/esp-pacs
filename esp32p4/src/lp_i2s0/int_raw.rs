#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `RX_DONE` reader - The raw interrupt status bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_R = crate::BitReader;
#[doc = "Field `RX_HUNG` reader - The raw interrupt status bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_R = crate::BitReader;
#[doc = "Field `RX_FIFOMEM_UDF` reader - The raw interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RX_FIFOMEM_UDF_R = crate::BitReader;
#[doc = "Field `VAD_DONE` reader - The raw interrupt status bit for the vad_done_int interrupt"]
pub type VAD_DONE_R = crate::BitReader;
#[doc = "Field `VAD_RESET_DONE` reader - The raw interrupt status bit for the vad_reset_done_int interrupt"]
pub type VAD_RESET_DONE_R = crate::BitReader;
#[doc = "Field `RX_MEM_THRESHOLD` reader - The raw interrupt status bit for the rx_mem_threshold_int interrupt"]
pub type RX_MEM_THRESHOLD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf(&self) -> RX_FIFOMEM_UDF_R {
        RX_FIFOMEM_UDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn vad_done(&self) -> VAD_DONE_R {
        VAD_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn vad_reset_done(&self) -> VAD_RESET_DONE_R {
        VAD_RESET_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold(&self) -> RX_MEM_THRESHOLD_R {
        RX_MEM_THRESHOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("rx_done", &format_args!("{}", self.rx_done().bit()))
            .field("rx_hung", &format_args!("{}", self.rx_hung().bit()))
            .field(
                "rx_fifomem_udf",
                &format_args!("{}", self.rx_fifomem_udf().bit()),
            )
            .field("vad_done", &format_args!("{}", self.vad_done().bit()))
            .field(
                "vad_reset_done",
                &format_args!("{}", self.vad_reset_done().bit()),
            )
            .field(
                "rx_mem_threshold",
                &format_args!("{}", self.rx_mem_threshold().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "I2S interrupt raw register, valid in level.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
