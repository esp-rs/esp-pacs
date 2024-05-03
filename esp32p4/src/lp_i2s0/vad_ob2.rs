#[doc = "Register `VAD_OB2` reader"]
pub type R = crate::R<VAD_OB2_SPEC>;
#[doc = "Field `NOISE_AMP_OB` reader - Reg noise_amp observe signal"]
pub type NOISE_AMP_OB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reg noise_amp observe signal"]
    #[inline(always)]
    pub fn noise_amp_ob(&self) -> NOISE_AMP_OB_R {
        NOISE_AMP_OB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_OB2")
            .field("noise_amp_ob", &self.noise_amp_ob().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VAD_OB2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_ob2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_OB2_SPEC;
impl crate::RegisterSpec for VAD_OB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob2::R`](R) reader structure"]
impl crate::Readable for VAD_OB2_SPEC {}
#[doc = "`reset()` method sets VAD_OB2 to value 0"]
impl crate::Resettable for VAD_OB2_SPEC {
    const RESET_VALUE: u32 = 0;
}
