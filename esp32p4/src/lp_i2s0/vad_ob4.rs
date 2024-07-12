#[doc = "Register `VAD_OB4` reader"]
pub type R = crate::R<VAD_OB4_SPEC>;
#[doc = "Field `NOISE_STD_OB` reader - Reg noise_std observe signal"]
pub type NOISE_STD_OB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reg noise_std observe signal"]
    #[inline(always)]
    pub fn noise_std_ob(&self) -> NOISE_STD_OB_R {
        NOISE_STD_OB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_OB4")
            .field("noise_std_ob", &self.noise_std_ob())
            .finish()
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_OB4_SPEC;
impl crate::RegisterSpec for VAD_OB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob4::R`](R) reader structure"]
impl crate::Readable for VAD_OB4_SPEC {}
#[doc = "`reset()` method sets VAD_OB4 to value 0"]
impl crate::Resettable for VAD_OB4_SPEC {
    const RESET_VALUE: u32 = 0;
}
