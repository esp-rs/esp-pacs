#[doc = "Register `VAD_OB3` reader"]
pub type R = crate::R<VAD_OB3_SPEC>;
#[doc = "Field `NOISE_MEAN_OB` reader - Reg noise_mean observe signal"]
pub type NOISE_MEAN_OB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reg noise_mean observe signal"]
    #[inline(always)]
    pub fn noise_mean_ob(&self) -> NOISE_MEAN_OB_R {
        NOISE_MEAN_OB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_OB3")
            .field(
                "noise_mean_ob",
                &format_args!("{}", self.noise_mean_ob().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VAD_OB3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_ob3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_OB3_SPEC;
impl crate::RegisterSpec for VAD_OB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob3::R`](R) reader structure"]
impl crate::Readable for VAD_OB3_SPEC {}
#[doc = "`reset()` method sets VAD_OB3 to value 0"]
impl crate::Resettable for VAD_OB3_SPEC {
    const RESET_VALUE: u32 = 0;
}
