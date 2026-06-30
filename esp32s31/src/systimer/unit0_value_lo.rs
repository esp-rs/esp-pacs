#[doc = "Register `UNIT0_VALUE_LO` reader"]
pub type R = crate::R<UNIT0_VALUE_LO_SPEC>;
#[doc = "Field `TIMER_UNIT0_VALUE_LO` reader - Represents UNIT0 read value, low 32 bits."]
pub type TIMER_UNIT0_VALUE_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents UNIT0 read value, low 32 bits."]
    #[inline(always)]
    pub fn timer_unit0_value_lo(&self) -> TIMER_UNIT0_VALUE_LO_R {
        TIMER_UNIT0_VALUE_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT0_VALUE_LO")
            .field("timer_unit0_value_lo", &self.timer_unit0_value_lo())
            .finish()
    }
}
#[doc = "UNIT0 value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`unit0_value_lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT0_VALUE_LO_SPEC;
impl crate::RegisterSpec for UNIT0_VALUE_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unit0_value_lo::R`](R) reader structure"]
impl crate::Readable for UNIT0_VALUE_LO_SPEC {}
#[doc = "`reset()` method sets UNIT0_VALUE_LO to value 0"]
impl crate::Resettable for UNIT0_VALUE_LO_SPEC {}
