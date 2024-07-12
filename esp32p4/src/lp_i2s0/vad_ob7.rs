#[doc = "Register `VAD_OB7` reader"]
pub type R = crate::R<VAD_OB7_SPEC>;
#[doc = "Field `ENERGY_LOW_OB` reader - Reg energy bit 31~0 observe signal"]
pub type ENERGY_LOW_OB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reg energy bit 31~0 observe signal"]
    #[inline(always)]
    pub fn energy_low_ob(&self) -> ENERGY_LOW_OB_R {
        ENERGY_LOW_OB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_OB7")
            .field("energy_low_ob", &self.energy_low_ob())
            .finish()
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_OB7_SPEC;
impl crate::RegisterSpec for VAD_OB7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob7::R`](R) reader structure"]
impl crate::Readable for VAD_OB7_SPEC {}
#[doc = "`reset()` method sets VAD_OB7 to value 0"]
impl crate::Resettable for VAD_OB7_SPEC {
    const RESET_VALUE: u32 = 0;
}
