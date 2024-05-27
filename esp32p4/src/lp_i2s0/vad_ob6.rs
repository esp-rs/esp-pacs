#[doc = "Register `VAD_OB6` reader"]
pub type R = crate::R<VAD_OB6_SPEC>;
#[doc = "Field `THRESHOLD_OB` reader - Reg threshold observe signal"]
pub type THRESHOLD_OB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reg threshold observe signal"]
    #[inline(always)]
    pub fn threshold_ob(&self) -> THRESHOLD_OB_R {
        THRESHOLD_OB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_OB6")
            .field("threshold_ob", &self.threshold_ob())
            .finish()
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_ob6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_OB6_SPEC;
impl crate::RegisterSpec for VAD_OB6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob6::R`](R) reader structure"]
impl crate::Readable for VAD_OB6_SPEC {}
#[doc = "`reset()` method sets VAD_OB6 to value 0"]
impl crate::Resettable for VAD_OB6_SPEC {
    const RESET_VALUE: u32 = 0;
}
