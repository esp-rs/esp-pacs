#[doc = "Register `CHNL0_OUT_CNT` reader"]
pub type R = crate::R<CHNL0_OUT_CNT_SPEC>;
#[doc = "Field `CHNL0_OUT_CNT` reader - Represents the bytes numbers send to the DMA when EOF occurs."]
pub type CHNL0_OUT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Represents the bytes numbers send to the DMA when EOF occurs."]
    #[inline(always)]
    pub fn chnl0_out_cnt(&self) -> CHNL0_OUT_CNT_R {
        CHNL0_OUT_CNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_OUT_CNT")
            .field("chnl0_out_cnt", &self.chnl0_out_cnt())
            .finish()
    }
}
#[doc = "Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_out_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_OUT_CNT_SPEC;
impl crate::RegisterSpec for CHNL0_OUT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_out_cnt::R`](R) reader structure"]
impl crate::Readable for CHNL0_OUT_CNT_SPEC {}
#[doc = "`reset()` method sets CHNL0_OUT_CNT to value 0"]
impl crate::Resettable for CHNL0_OUT_CNT_SPEC {}
