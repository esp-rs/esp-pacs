#[doc = "Register `CHNL0_TRACE1` reader"]
pub type R = crate::R<CHNL0_TRACE1_SPEC>;
#[doc = "Field `CHNL0_OUT_INC` reader - Represents the samples numbers send to the DMA"]
pub type CHNL0_OUT_INC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Represents the samples numbers send to the DMA"]
    #[inline(always)]
    pub fn chnl0_out_inc(&self) -> CHNL0_OUT_INC_R {
        CHNL0_OUT_INC_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_TRACE1")
            .field("chnl0_out_inc", &self.chnl0_out_inc())
            .finish()
    }
}
#[doc = "Debug Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_trace1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_TRACE1_SPEC;
impl crate::RegisterSpec for CHNL0_TRACE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_trace1::R`](R) reader structure"]
impl crate::Readable for CHNL0_TRACE1_SPEC {}
#[doc = "`reset()` method sets CHNL0_TRACE1 to value 0"]
impl crate::Resettable for CHNL0_TRACE1_SPEC {}
