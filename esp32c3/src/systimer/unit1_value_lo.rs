#[doc = "Register `UNIT1_VALUE_LO` reader"]
pub struct R(crate::R<UNIT1_VALUE_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIT1_VALUE_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIT1_VALUE_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIT1_VALUE_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_UNIT1_VALUE_LO` reader - timer read value low 32bit"]
pub type TIMER_UNIT1_VALUE_LO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - timer read value low 32bit"]
    #[inline(always)]
    pub fn timer_unit1_value_lo(&self) -> TIMER_UNIT1_VALUE_LO_R {
        TIMER_UNIT1_VALUE_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT1_VALUE_LO")
            .field(
                "timer_unit1_value_lo",
                &format_args!("{}", self.timer_unit1_value_lo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT1_VALUE_LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SYSTIMER_UNIT1_VALUE_LO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unit1_value_lo](index.html) module"]
pub struct UNIT1_VALUE_LO_SPEC;
impl crate::RegisterSpec for UNIT1_VALUE_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unit1_value_lo::R](R) reader structure"]
impl crate::Readable for UNIT1_VALUE_LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UNIT1_VALUE_LO to value 0"]
impl crate::Resettable for UNIT1_VALUE_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
