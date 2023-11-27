#[doc = "Register `VAD_RESULT` reader"]
pub type R = crate::R<VAD_RESULT_SPEC>;
#[doc = "Field `VAD_FLAG` reader - Reg vad flag observe signal"]
pub type VAD_FLAG_R = crate::BitReader;
#[doc = "Field `ENERGY_ENOUGH` reader - Reg energy enough observe signal"]
pub type ENERGY_ENOUGH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reg vad flag observe signal"]
    #[inline(always)]
    pub fn vad_flag(&self) -> VAD_FLAG_R {
        VAD_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reg energy enough observe signal"]
    #[inline(always)]
    pub fn energy_enough(&self) -> ENERGY_ENOUGH_R {
        ENERGY_ENOUGH_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_RESULT")
            .field("vad_flag", &format_args!("{}", self.vad_flag().bit()))
            .field(
                "energy_enough",
                &format_args!("{}", self.energy_enough().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VAD_RESULT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "I2S VAD Result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_result::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_RESULT_SPEC;
impl crate::RegisterSpec for VAD_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_result::R`](R) reader structure"]
impl crate::Readable for VAD_RESULT_SPEC {}
#[doc = "`reset()` method sets VAD_RESULT to value 0"]
impl crate::Resettable for VAD_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
