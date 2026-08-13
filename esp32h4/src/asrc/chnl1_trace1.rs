#[doc = "Register `CHNL1_TRACE1` reader"]
pub type R = crate::R<CHNL1_TRACE1_SPEC>;
#[doc = "Field `CHNL1_OUT_INC` reader - Represents the data byte numbers send to the DMA"]
pub type CHNL1_OUT_INC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Represents the data byte numbers send to the DMA"]
    #[inline(always)]
    pub fn chnl1_out_inc(&self) -> CHNL1_OUT_INC_R {
        CHNL1_OUT_INC_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL1_TRACE1")
            .field("chnl1_out_inc", &self.chnl1_out_inc())
            .finish()
    }
}
#[doc = "Debug Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_trace1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_TRACE1_SPEC;
impl crate::RegisterSpec for CHNL1_TRACE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl1_trace1::R`](R) reader structure"]
impl crate::Readable for CHNL1_TRACE1_SPEC {}
#[doc = "`reset()` method sets CHNL1_TRACE1 to value 0"]
impl crate::Resettable for CHNL1_TRACE1_SPEC {}
